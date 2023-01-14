use ergo_lib::ergotree_ir::types::stype;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct SType(stype::SType);

impl From<&SType> for stype::SType {
    fn from(s: &SType) -> Self {
        s.0.clone()
    }
}

#[wasm_bindgen]
impl SType {
    pub fn unit() -> SType {
        SType(stype::SType::SUnit)
    }

    pub fn boolean() -> SType {
        SType(stype::SType::SBoolean)
    }

    pub fn byte() -> SType {
        SType(stype::SType::SByte)
    }

    pub fn short() -> SType {
        SType(stype::SType::SShort)
    }

    pub fn int() -> SType {
        SType(stype::SType::SInt)
    }

    pub fn long() -> SType {
        SType(stype::SType::SLong)
    }

    pub fn bigint() -> SType {
        SType(stype::SType::SBigInt)
    }

    #[wasm_bindgen(js_name = sigmaProp)]
    pub fn sigma_prop() -> SType {
        SType(stype::SType::SSigmaProp)
    }

    #[wasm_bindgen(js_name = groupElement)]
    pub fn group_element() -> SType {
        SType(stype::SType::SGroupElement)
    }

    #[wasm_bindgen(js_name = ergoBox)]
    pub fn ergo_box() -> SType {
        SType(stype::SType::SBox)
    }

    pub fn coll(tpe: &SType) -> SType {
        let coll_type = tpe.into();
        let inner = stype::SType::SColl(Box::new(coll_type));

        SType(inner)
    }

    pub fn tuple(a: &SType, b: &SType) -> SType {
        // unwrap is safe, explicitly using 2 elements, no chance of out-of-bounds for bounded vec.
        let tuple = vec![a.into(), b.into()].try_into().unwrap();
        let inner = stype::SType::STuple(tuple);

        SType(inner)
    }
}
