use derive_more::{From, Into};
use ergo_lib_utils::extract_classname;
use ergo_wasm_derive::TryFromJsValue;
use js_sys::Uint8Array;
use scorex_crypto_avltree::operation::{KeyValue, Operation as NativeOperation};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        typescript_type = "LookupOperation | InsertOperation | RemoveOperation | UpdateOperation"
    )]
    pub type TsOperationType;
}

#[derive(Debug, Clone)]
pub(crate) enum Operation {
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
}
