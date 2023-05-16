//! Macros to help with missing functionality in `wasm_bindgen`.
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

/// Implementation of [`TryFromJsValue`] mirrored from here [`wasm-bindgen-derive`](https://github.com/fjarri/wasm-bindgen-derive/blob/master/src/lib.rs)
/// It serves as a basis for workarounds for some lapses of functionality in [`wasm-bindgen`](https://crates.io/crates/wasm-bindgen).
///
/// [`TryFromJsValue`] is needed to lift `JsValue` types into their `#[wasm_bindgen]` counter-parts. This is needed particularly for
/// working with accepting & returning arrays of wasm types across the JS <-> Rust boundary.
///
/// Derives a `TryFrom<&JsValue>` for a type exported using `#[wasm_bindgen]`.
///
/// Note that:
///  - this derivation must be be positioned before `#[wasm_bindgen]`;
///  - the type must implement [`Clone`].
///
/// The macro is authored by [**@AlexKorn**](https://github.com/AlexKorn)
/// based on the idea of [**@aweinstock314**](https://github.com/aweinstock314).
/// See [this](https://github.com/rustwasm/wasm-bindgen/issues/2231#issuecomment-656293288)
/// and [this](https://github.com/rustwasm/wasm-bindgen/issues/2231#issuecomment-1169658111)
/// GitHub comments.
///
/// ## Optional arguments
///
/// `wasm-bindgen` supports method arguments of the form `Option<T>`,
/// where `T` is an exported type, but it has an unexpected side effect on the JS side:
/// the value passed to a method this way gets consumed (mimicking Rust semantics).
/// See [this issue](https://github.com/rustwasm/wasm-bindgen/issues/2370).
/// `Option<&T>` is not currently supported, but an equivalent behavior can be implemented manually.
///
/// ```
/// use js_sys::Error;
/// use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
/// use ergo_wasm_derive::TryFromJsValue;
///
/// // Derive `TryFromJsValue` for the target structure (note that it has to come
/// // before the `[#wasm_bindgen]` attribute, and requires `Clone`):
/// #[derive(TryFromJsValue)]
/// #[wasm_bindgen]
/// #[derive(Clone)]
/// struct MyType(usize);
///
/// // To have a correct typing annotation generated for TypeScript, declare a custom type.
/// #[wasm_bindgen]
/// extern "C" {
///     #[wasm_bindgen(typescript_type = "MyType | null")]
///     pub type OptionMyType;
/// }
///
/// // Use this type in the function signature.
/// pub fn foo(value: &OptionMyType) -> Result<usize, JsValue> {
///     let js_value: &JsValue = value.as_ref();
///     let typed_value: Option<MyType> = if js_value.is_null() {
///         None
///     } else {
///         MyType::try_from(js_value).ok()
///     };
///
///     // Use the typed value
///     Ok(typed_value.map(|value| value.0).unwrap_or_default())
/// }
/// ```
///
/// ## Vector arguments
///
/// `wasm-bindgen` currently does not support vector arguments with elements having an exported type.
/// See [this issue](https://github.com/rustwasm/wasm-bindgen/issues/111),
/// which, although it is mainly about returning vectors, will probably allow taking vectors too
/// when fixed.
///
/// The workaround is similar to that for the optional arguments, with one step added,
/// where we try to cast the [`JsValue`](`wasm_bindgen::JsValue`) into [`Array`](`js_sys::Array`).
/// The following example also shows how to return an array with elements having an exported type.
///
/// ```
/// use js_sys::Error;
/// use wasm_bindgen::JsCast;
/// use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
/// use ergo_wasm_derive::TryFromJsValue;
///
/// #[derive(TryFromJsValue)]
/// #[wasm_bindgen]
/// #[derive(Clone)]
/// pub struct MyType(usize);
///
/// // To have a correct typing annotation generated for TypeScript, declare a custom type.
/// #[wasm_bindgen]
/// extern "C" {
///     #[wasm_bindgen(typescript_type = "MyType[]")]
///     pub type MyTypeArray;
/// }
///
/// // Use this type in the function signature.
/// pub fn foo(val: &MyTypeArray) -> Result<MyTypeArray, Error> {
///
///    // Unpack the array
///
///     let js_val: &JsValue = val.as_ref();
///     if !js_sys::Array::is_array(js_val) {
///         return Err(Error::new("The argument must be an array"));
///     }
///     let array = js_sys::Array::from(js_val);
///     let length: usize = array.length().try_into().map_err(|err| Error::new(&format!("{}", err)))?;
///     let mut typed_array = Vec::<MyType>::with_capacity(length);
///     for js in array.iter() {
///         let typed_elem = MyType::try_from(&js)?;
///         typed_array.push(typed_elem);
///     }
///
///     // Now we have `typed_array: Vec<MyType>`.
///
///     // Return the array
///
///     Ok(typed_array
///         .into_iter()
///         .map(JsValue::from)
///         .collect::<js_sys::Array>()
///         .unchecked_into::<MyTypeArray>())
/// }
/// ```
#[proc_macro_derive(TryFromJsValue)]
pub fn derive_try_from_jsvalue(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    match data {
        Data::Struct(_) => {}
        _ => return derive_error!("TryFromJsValue may only be derived on structs"),
    };

    let wasm_bindgen_meta = input.attrs.iter().find_map(|attr| {
        attr.parse_meta()
            .ok()
            .and_then(|meta| match meta.path().is_ident("wasm_bindgen") {
                true => Some(meta),
                false => None,
            })
    });
    if wasm_bindgen_meta.is_none() {
        return derive_error!(
            "TryFromJsValue can be defined only on struct exported to wasm with #[wasm_bindgen]"
        );
    }

    let maybe_js_class = wasm_bindgen_meta
        .and_then(|meta| match meta {
            syn::Meta::List(list) => Some(list),
            _ => None,
        })
        .and_then(|meta_list| {
            meta_list.nested.iter().find_map(|nested_meta| {
                let maybe_meta = match nested_meta {
                    syn::NestedMeta::Meta(meta) => Some(meta),
                    _ => None,
                };

                maybe_meta
                    .and_then(|meta| match meta {
                        syn::Meta::NameValue(name_value) => Some(name_value),
                        _ => None,
                    })
                    .and_then(|name_value| match name_value.path.is_ident("js_name") {
                        true => Some(name_value.lit.clone()),
                        false => None,
                    })
                    .and_then(|lit| match lit {
                        syn::Lit::Str(str) => Some(str.value()),
                        _ => None,
                    })
            })
        });

    let wasm_bindgen_macro_invocaton = match maybe_js_class {
        Some(class) => format!("wasm_bindgen(js_class = \"{}\")", class),
        None => "wasm_bindgen".to_string(),
    }
    .parse::<TokenStream2>()
    .unwrap();

    let expanded = quote! {
        impl #name {
            pub fn __get_classname() -> &'static str {
                ::core::stringify!(#name)
            }
        }

        #[#wasm_bindgen_macro_invocaton]
        impl #name {
            #[wasm_bindgen(js_name = "__getClassname")]
            pub fn __js_get_classname(&self) -> String {
                ::core::stringify!(#name).to_owned()
            }
        }

        impl ::core::convert::TryFrom<&::wasm_bindgen::JsValue> for #name {
            type Error = ::wasm_bindgen::JsValue;

            fn try_from(js: &::wasm_bindgen::JsValue) -> Result<Self, Self::Error> {
                use ::wasm_bindgen::JsCast;
                use ::wasm_bindgen::convert::RefFromWasmAbi;

                let classname = Self::__get_classname();

                if !js.is_object() {
                    return Err(::wasm_bindgen::JsValue::from_str(format!("Value supplied as {} is not an object", classname).as_str()));
                }

                let no_get_classname_msg = concat!(
                    "no __getClassname method specified for object; ",
                    "did you forget to derive TryFromJsObject for this type?");
                let get_classname = ::js_sys::Reflect::get(
                    js,
                    &::wasm_bindgen::JsValue::from("__getClassname"),
                )
                .or(Err(::wasm_bindgen::JsValue::from_str(no_get_classname_msg)))?;

                if get_classname.is_undefined() {
                    return Err(::wasm_bindgen::JsValue::from_str(no_get_classname_msg));
                }

                let get_classname = get_classname
                    .dyn_into::<::js_sys::Function>()
                    .map_err(|err| ::wasm_bindgen::JsValue::from_str(format!("__getClassname is not a function, {:?}", err).as_str()))?;

                let object_classname: String = ::js_sys::Reflect::apply(
                        &get_classname,
                        js,
                        &::js_sys::Array::new(),
                    )
                    .ok()
                    .and_then(|v| v.as_string())
                    .ok_or_else(|| ::wasm_bindgen::JsValue::from_str("Failed to get classname"))?;

                if object_classname.as_str() == classname {
                    let ptr = ::js_sys::Reflect::get(js, &::wasm_bindgen::JsValue::from_str("__wbg_ptr"))
                        .map_err(|err| ::wasm_bindgen::JsValue::from_str(format!("{:?}", err).as_str()))?;
                    let ptr_u32: u32 = ptr.as_f64().ok_or(::wasm_bindgen::JsValue::NULL)
                        .map_err(|err| ::wasm_bindgen::JsValue::from_str(format!("{:?}", err).as_str()))?
                        as u32;
                    let instance_ref = unsafe { #name::ref_from_abi(ptr_u32) };
                    Ok(instance_ref.clone())
                } else {
                    Err(::wasm_bindgen::JsValue::from_str(format!("Cannot convert {} to {}", object_classname, classname).as_str()))
                }
            }
        }
    };

    TokenStream::from(expanded)
}
