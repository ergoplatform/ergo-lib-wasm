use crate::prelude::*;
use crate::{blockchain::ergo_box::ErgoBox, js::extract_classname};
use ergo_lib::ergo_chain_types::Base16DecodedBytes;
use ergo_lib::{
    ergo_chain_types::EcPoint,
    ergotree_ir::{
        base16_str::Base16Str,
        bigint256::BigInt256,
        chain::ergo_box::ErgoBox as NativeErgoBox,
        mir::{
            constant::{Constant, Literal},
            value::{CollKind, Value},
        },
        serialization::SigmaSerializable,
        sigma_protocol::sigma_boolean::{SigmaBoolean, SigmaProp},
        types::stype::SType,
    },
};
use ergo_wasm_derive::TryFromJsValue;
use num_traits::Num;

use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::*;

/// Like `LiftIntoSType` this trait provides a method to get the `stype` from a type
/// but is intended to be used at runtime.
///
/// Mostly for JS array values - there is no way to
/// know at compile time what the `stype` of the array elements
/// will be so we need to retrieve the type dynamically.
trait DynLiftIntoSType {
    fn dyn_stype(&self) -> SType;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        typescript_type = "SBoolean | SByte | SShort | SInt | SLong | SBigInt | SSigmaProp | SGroupElement | SErgoBox | SColl"
    )]
    pub type TsSLiteralType;

    #[wasm_bindgen(
        typescript_type = "SBoolean[] | SByte[] | SShort[] | SInt[] | SLong[] | SBigInt[] | SSigmaProp[] | SGroupElement[] | SErgoBox[] | SColl[]"
    )]
    pub type TsSLiteralArrayType;
}

#[wasm_bindgen]
pub struct SConstant {
    inner: Constant,
    /// The `JsValue` used to create this `SConstant`.
    ///
    /// For example a class instance of `SByte` / `SColl` / etc.
    /// Kept alive so we can retrieve the JS value used to
    /// create this constant.
    created_from: JsValue,
}

#[wasm_bindgen]
impl SConstant {
    #[wasm_bindgen(js_name = toHex)]
    pub fn to_hex(&self) -> Result<String, JsValue> {
        self.inner.base16_str().map_err_js_value()
    }

    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(hex: &str) -> Result<SConstant, JsValue> {
        let bytes = Base16DecodedBytes::try_from(hex).map_err_js_value()?;
        let inner = Constant::try_from(bytes).map_err_js_value()?;

        Ok(SConstant {
            inner,
            created_from: hex.into(),
        })
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &Uint8Array) -> Result<SConstant, JsValue> {
        let inner = Constant::try_from(bytes.to_vec()).map_err_js_value()?;

        Ok(SConstant {
            inner,
            created_from: bytes.into(),
        })
    }

    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
        self.inner.sigma_serialize_bytes().map_err_js_value()
    }

    #[wasm_bindgen(getter, js_name = createdFrom)]
    pub fn created_from(&self) -> JsValue {
        self.created_from.clone()
    }

    #[wasm_bindgen(getter, js_name = typeStr)]
    pub fn tpe(&self) -> String {
        format!("{:?}", self.inner.tpe)
    }

    #[wasm_bindgen]
    pub fn dbg(&self) -> String {
        format!("{:?}", self.inner)
    }
}

#[derive(Debug, Clone)]
pub enum SLiteral {
    Unit(SUnit),
    Boolean(SBoolean),
    Byte(SByte),
    Short(SShort),
    Int(SInt),
    Long(SLong),
    BigInt(SBigInt),
    SigmaProp(SSigmaProp),
    GroupElement(SGroupElement),
    ErgoBox(SErgoBox),
    Coll(SColl),
}

impl DynLiftIntoSType for SLiteral {
    fn dyn_stype(&self) -> SType {
        match self {
            SLiteral::Unit(_) => SType::SUnit,
            SLiteral::Boolean(_) => SType::SBoolean,
            SLiteral::Byte(_) => SType::SByte,
            SLiteral::Short(_) => SType::SShort,
            SLiteral::Int(_) => SType::SInt,
            SLiteral::Long(_) => SType::SLong,
            SLiteral::BigInt(_) => SType::SBigInt,
            SLiteral::SigmaProp(_) => SType::SSigmaProp,
            SLiteral::GroupElement(_) => SType::SGroupElement,
            SLiteral::ErgoBox(_) => SType::SBox,
            SLiteral::Coll(v) => v.dyn_stype(),
        }
    }
}

impl TryFrom<JsValue> for SLiteral {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let object_classname = extract_classname(&value)?;

        let vref = value.as_ref();
        match object_classname.as_str() {
            "SUnit" => Ok(SLiteral::Unit(vref.try_into()?)),
            "SBoolean" => Ok(SLiteral::Boolean(vref.try_into()?)),
            "SByte" => Ok(SLiteral::Byte(vref.try_into()?)),
            "SShort" => Ok(SLiteral::Short(vref.try_into()?)),
            "SInt" => Ok(SLiteral::Int(vref.try_into()?)),
            "SLong" => Ok(SLiteral::Long(vref.try_into()?)),
            "SBigInt" => Ok(SLiteral::BigInt(vref.try_into()?)),
            "SSigmaProp" => Ok(SLiteral::SigmaProp(vref.try_into()?)),
            "SGroupElement" => Ok(SLiteral::GroupElement(vref.try_into()?)),
            "SErgoBox" => Ok(SLiteral::ErgoBox(vref.try_into()?)),
            "SColl" => Ok(SLiteral::Coll(vref.try_into()?)),
            _ => Err(format!("SLiteral: unknown class '{}'", object_classname).into()),
        }
    }
}

impl From<SLiteral> for Literal {
    fn from(v: SLiteral) -> Self {
        match v {
            SLiteral::Unit(v) => v.into(),
            SLiteral::Boolean(v) => v.into(),
            SLiteral::Byte(v) => v.into(),
            SLiteral::Short(v) => v.into(),
            SLiteral::Int(v) => v.into(),
            SLiteral::Long(v) => v.into(),
            SLiteral::BigInt(v) => v.into(),
            SLiteral::SigmaProp(v) => v.into(),
            SLiteral::GroupElement(v) => v.into(),
            SLiteral::ErgoBox(v) => v.into(),
            SLiteral::Coll(v) => v.into(),
        }
    }
}

impl TryFrom<SLiteral> for Constant {
    type Error = JsValue;

    fn try_from(v: SLiteral) -> Result<Self, Self::Error> {
        Value::from(Literal::from(v)).try_into().map_err_js_value()
    }
}

macro_rules! impl_primitive_constant_literal {
    ($l:ident, $ty:tt) => {
        #[derive(TryFromJsValue)]
        #[wasm_bindgen]
        #[derive(Debug, Clone)]
        pub struct $l($ty);

        #[wasm_bindgen]
        impl $l {
            #[wasm_bindgen(constructor)]
            pub fn new(value: $ty) -> Self {
                Self(value)
            }

            #[wasm_bindgen(getter)]
            pub fn value(&self) -> $ty {
                self.0.clone()
            }

            #[wasm_bindgen(js_name = intoConstant)]
            pub fn into_constant(self) -> SConstant {
                SConstant {
                    inner: self.0.into(),
                    created_from: self.into(),
                }
            }
        }

        impl From<$l> for Literal {
            fn from(value: $l) -> Self {
                value.0.into()
            }
        }
    };
}

impl_primitive_constant_literal!(SBoolean, bool);
impl_primitive_constant_literal!(SByte, i8);
impl_primitive_constant_literal!(SShort, i16);
impl_primitive_constant_literal!(SInt, i32);

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone, Default)]
pub struct SUnit;

#[wasm_bindgen]
impl SUnit {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SUnit {
        Self
    }

    #[wasm_bindgen(js_name = intoConstant)]
    pub fn into_constant(self) -> Result<SConstant, JsValue> {
        Ok(SConstant {
            inner: ().into(),
            created_from: self.into(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        JsValue::NULL
    }
}

impl From<SUnit> for Literal {
    fn from(_: SUnit) -> Self {
        Literal::Unit
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct SLong(i64);

#[wasm_bindgen]
impl SLong {
    #[wasm_bindgen(constructor)]
    pub fn new(value: js_sys::BigInt) -> Result<SLong, JsValue> {
        Ok(Self(value.try_into().map_err_js_value()?))
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> js_sys::BigInt {
        js_sys::BigInt::from(self.0)
    }

    #[wasm_bindgen(js_name = intoConstant)]
    pub fn into_constant(self) -> SConstant {
        SConstant {
            inner: self.0.into(),
            created_from: self.into(),
        }
    }
}

impl From<SLong> for Literal {
    fn from(value: SLong) -> Self {
        value.0.into()
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct SBigInt {
    value: BigInt256,
    js_value: js_sys::BigInt,
}

#[wasm_bindgen]
impl SBigInt {
    #[wasm_bindgen(constructor)]
    pub fn new(js_value: js_sys::BigInt) -> Result<SBigInt, JsValue> {
        let radix = 10;
        let s = js_value
            .to_string(radix)
            .map_err_js_value()?
            .as_string()
            .ok_or_else(|| JsValue::from_str("failed to convert JS string"))?;
        let bi = BigInt256::from_str_radix(&s, radix as u32).map_err_js_value()?;

        Ok(SBigInt {
            value: bi,
            js_value,
        })
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> js_sys::BigInt {
        self.js_value.clone()
    }

    #[wasm_bindgen(js_name = intoConstant)]
    pub fn into_constant(self) -> SConstant {
        SConstant {
            inner: self.value.clone().into(),
            created_from: self.into(),
        }
    }
}

impl From<SBigInt> for Literal {
    fn from(bigint: SBigInt) -> Self {
        bigint.value.into()
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct SSigmaProp {
    value: SigmaProp,
    js_value: JsValue,
}

#[wasm_bindgen]
impl SSigmaProp {
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: &str) -> Result<SSigmaProp, JsValue> {
        let value: SigmaBoolean = serde_json::from_str(json).map_err_js_value()?;

        Ok(SSigmaProp {
            value: value.into(),
            js_value: json.into(),
        })
    }

    #[wasm_bindgen(js_name = fromBool)]
    pub fn from_bool(bool: bool) -> SSigmaProp {
        SSigmaProp {
            value: SigmaProp::new(bool.into()),
            js_value: bool.into(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        self.js_value.clone()
    }

    #[wasm_bindgen(js_name = intoConstant)]
    pub fn into_constant(self) -> SConstant {
        SConstant {
            inner: self.value.clone().into(),
            created_from: self.into(),
        }
    }
}

impl From<SSigmaProp> for Literal {
    fn from(prop: SSigmaProp) -> Self {
        prop.value.into()
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct SGroupElement {
    value: EcPoint,
    js_value: JsValue,
}

#[wasm_bindgen]
impl SGroupElement {
    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(hex: &str) -> Result<SGroupElement, JsValue> {
        Ok(SGroupElement {
            value: hex.to_string().try_into().map_err_js_value()?,
            js_value: hex.into(),
        })
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &Uint8Array) -> Result<SGroupElement, JsValue> {
        Ok(SGroupElement {
            value: EcPoint::sigma_parse_bytes(&bytes.to_vec()).map_err_js_value()?,
            js_value: bytes.into(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        self.js_value.clone()
    }

    #[wasm_bindgen(js_name = intoConstant)]
    pub fn into_constant(self) -> SConstant {
        SConstant {
            inner: self.value.clone().into(),
            created_from: self.into(),
        }
    }
}

impl From<SGroupElement> for Literal {
    fn from(prop: SGroupElement) -> Self {
        prop.value.into()
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct SErgoBox {
    value: NativeErgoBox,
    js_value: ErgoBox,
}

#[wasm_bindgen]
impl SErgoBox {
    #[wasm_bindgen(constructor)]
    pub fn new(ergo_box: &ErgoBox) -> SErgoBox {
        SErgoBox {
            value: ergo_box.clone().into(),
            js_value: ergo_box.clone(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> ErgoBox {
        self.js_value.clone()
    }

    #[wasm_bindgen(js_name = intoConstant)]
    pub fn into_constant(self) -> SConstant {
        SConstant {
            inner: self.value.clone().into(),
            created_from: self.into(),
        }
    }
}

impl From<SErgoBox> for Literal {
    fn from(prop: SErgoBox) -> Self {
        prop.value.into()
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct SColl {
    values: Vec<SLiteral>,
    js_value: Array,
}

#[wasm_bindgen]
impl SColl {
    #[wasm_bindgen(constructor)]
    pub fn new(js_coll: &TsSLiteralArrayType) -> Result<SColl, JsValue> {
        // TODO: should we validate the js array input?
        // I.e ensure the following:
        //  - at least 1 element (?)
        //  - ensure all elements of same type
        let mut coll: Vec<SLiteral> = vec![];
        let arr = Array::from(js_coll);

        for js in arr.iter() {
            coll.push(js.try_into()?)
        }

        Ok(SColl {
            values: coll,
            js_value: arr,
        })
    }

    #[wasm_bindgen(js_name = intoConstant)]
    pub fn into_constant(self) -> Result<SConstant, JsValue> {
        let inner: Constant = Value::from(Literal::from(self.clone()))
            .try_into()
            .map_err_js_value()?;

        Ok(SConstant {
            inner,
            created_from: self.into(),
        })
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> Array {
        self.js_value.clone()
    }
}

impl SColl {
    fn elem_tpe(&self) -> SType {
        // when creating a `SColl` we should check that it is valid
        // in the constructor so `unwrap()` is safe here
        self.values.get(0).unwrap().dyn_stype()
    }
}

impl DynLiftIntoSType for SColl {
    fn dyn_stype(&self) -> SType {
        SType::SColl(Box::new(self.elem_tpe()))
    }
}

impl From<SColl> for Literal {
    fn from(value: SColl) -> Self {
        let items = value
            .values
            .iter()
            .map(|v| v.clone().into())
            .collect::<Vec<Literal>>();

        Literal::Coll(CollKind::WrappedColl {
            elem_tpe: value.elem_tpe(),
            items,
        })
    }
}
