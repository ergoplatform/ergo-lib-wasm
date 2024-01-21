use derive_more::{From, Into};
use ergo_lib_utils::{extract_classname, MapJsValueErrorResult};
use ergo_wasm_derive::TryFromJsValue;
use js_sys::Uint8Array;
use ergo_avltree_rust::operation::{KeyValue, Operation as NativeOperation};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        typescript_type = "LookupOperation | InsertOperation | RemoveOperation | UpdateOperation"
    )]
    pub type TsOperationType;
}

#[derive(Debug, Clone, From)]
pub enum Operation {
    Lookup(LookupOperation),
    Insert(InsertOperation),
    Remove(RemoveOperation),
    Update(UpdateOperation),
}

impl From<Operation> for NativeOperation {
    fn from(value: Operation) -> Self {
        match value {
            Operation::Lookup(op) => op.into(),
            Operation::Insert(op) => op.into(),
            Operation::Remove(op) => op.into(),
            Operation::Update(op) => op.into(),
        }
    }
}

impl TryFrom<JsValue> for Operation {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        let object_classname = extract_classname(&value)?;

        let vref = value.as_ref();
        match object_classname.as_str() {
            "LookupOperation" => Ok(Operation::Lookup(vref.try_into()?)),
            "InsertOperation" => Ok(Operation::Insert(vref.try_into()?)),
            "RemoveOperation" => Ok(Operation::Remove(vref.try_into()?)),
            "UpdateOperation" => Ok(Operation::Update(vref.try_into()?)),
            _ => Err(format!("Operation: unknown class '{}'", object_classname).into()),
        }
    }
}

impl From<Operation> for JsValue {
    fn from(value: Operation) -> Self {
        match value {
            Operation::Lookup(o) => o.into(),
            Operation::Insert(o) => o.into(),
            Operation::Remove(o) => o.into(),
            Operation::Update(o) => o.into(),
        }
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone, From, Into)]
pub struct LookupOperation(NativeOperation);

#[wasm_bindgen]
impl LookupOperation {
    #[wasm_bindgen(constructor)]
    pub fn new(key: Uint8Array) -> LookupOperation {
        LookupOperation(NativeOperation::Lookup(key.to_vec().into()))
    }

    /// Create instance using hex encoded strings.
    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(key: &str) -> Result<LookupOperation, JsValue> {
        Ok(LookupOperation(NativeOperation::Lookup(
            base16::decode(key).map_err_js_value()?.into(),
        )))
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone, From, Into)]
pub struct InsertOperation(NativeOperation);

#[wasm_bindgen]
impl InsertOperation {
    #[wasm_bindgen(constructor)]
    pub fn new(key: Uint8Array, value: Uint8Array) -> InsertOperation {
        let kv = KeyValue {
            key: key.to_vec().into(),
            value: value.to_vec().into(),
        };

        InsertOperation(NativeOperation::Insert(kv))
    }

    /// Create instance using hex encoded strings.
    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(key: &str, value: &str) -> Result<InsertOperation, JsValue> {
        let kv = KeyValue {
            key: base16::decode(key).map_err_js_value()?.into(),
            value: base16::decode(value).map_err_js_value()?.into(),
        };

        Ok(InsertOperation(NativeOperation::Insert(kv)))
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone, From, Into)]
pub struct RemoveOperation(NativeOperation);

#[wasm_bindgen]
impl RemoveOperation {
    #[wasm_bindgen(constructor)]
    pub fn new(key: Uint8Array) -> RemoveOperation {
        RemoveOperation(NativeOperation::Remove(key.to_vec().into()))
    }

    /// Create instance using hex encoded strings.
    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(key: &str) -> Result<RemoveOperation, JsValue> {
        Ok(RemoveOperation(NativeOperation::Remove(
            base16::decode(key).map_err_js_value()?.into(),
        )))
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone, From, Into)]
pub struct UpdateOperation(NativeOperation);

#[wasm_bindgen]
impl UpdateOperation {
    #[wasm_bindgen(constructor)]
    pub fn new(key: Uint8Array, value: Uint8Array) -> UpdateOperation {
        let kv = KeyValue {
            key: key.to_vec().into(),
            value: value.to_vec().into(),
        };

        UpdateOperation(NativeOperation::Update(kv))
    }

    /// Create instance using hex encoded strings.
    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(key: &str, value: &str) -> Result<UpdateOperation, JsValue> {
        let kv = KeyValue {
            key: base16::decode(key).map_err_js_value()?.into(),
            value: base16::decode(value).map_err_js_value()?.into(),
        };

        Ok(UpdateOperation(NativeOperation::Update(kv)))
    }
}

macro_rules! impl_key_method {
    ($($l:ident)*) => {$(
        #[wasm_bindgen]
        impl $l {
            /// The key of the operation.
            #[wasm_bindgen(getter)]
            pub fn key(self) -> Box<[u8]> {
                self.0.key().to_vec().into_boxed_slice()
            }
        }
    )*};
}

impl_key_method!(InsertOperation RemoveOperation UpdateOperation LookupOperation);

macro_rules! impl_value_method {
    ($($l:ident)*) => {$(
        #[wasm_bindgen]
        impl $l {
            /// The value of the operation.
            #[wasm_bindgen(getter)]
            pub fn value(self) -> Option<Box<[u8]>> {
                self.0.value().map(|b| b.to_vec().into_boxed_slice())
            }
        }
    )*};
}

impl_value_method!(InsertOperation UpdateOperation);
