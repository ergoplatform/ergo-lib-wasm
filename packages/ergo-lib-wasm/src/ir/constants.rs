pub mod literals;

use std::str::FromStr;

use crate::prelude::*;
use crate::{blockchain::ergo_box::ErgoBox, js::extract_classname};
use derive_more::{From, Into};
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

    pub type SLiteralLike;

    #[wasm_bindgen(method, getter, js_name = value)]
    fn get_value(this: &SLiteralLike) -> JsValue;
}

#[wasm_bindgen]
#[derive(Debug, Clone, From, Into)]
pub struct SConstant(Constant);

#[wasm_bindgen]
impl SConstant {
    /// Convert the {@link SConstant} to a hex string.
    #[wasm_bindgen(js_name = toHex)]
    pub fn to_hex(&self) -> Result<String, JsValue> {
        self.0.base16_str().map_err_js_value()
    }

    /// Create a {@link SConstant} from the provided hex string.
    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(hex: &str) -> Result<SConstant, JsValue> {
        let bytes = Base16DecodedBytes::try_from(hex).map_err_js_value()?;
        let inner = Constant::try_from(bytes).map_err_js_value()?;

        Ok(inner.into())
    }

    /// Create a {@link SConstant} from the provided byte array.
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &Uint8Array) -> Result<SConstant, JsValue> {
        let inner = Constant::try_from(bytes.to_vec()).map_err_js_value()?;

        Ok(inner.into())
    }

    /// Convert the {@link SConstant} to a byte array.
    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
        self.0.sigma_serialize_bytes().map_err_js_value()
    }

    /// Extract the literal value of this {@link SConstant} instance.
    #[wasm_bindgen(getter)]
    pub fn literal(&self) -> Result<TsSLiteralType, JsValue> {
        let v: JsValue = match self.0.v.clone() {
            Literal::Unit => SUnit::new().into(),
            Literal::Boolean(v) => SBoolean::new(v).into(),
            Literal::Byte(v) => SByte::new(v).into(),
            Literal::Short(v) => SShort::new(v).into(),
            Literal::Int(v) => SInt::new(v).into(),
            Literal::Long(v) => SLong::new(js_sys::BigInt::from(v))?.into(),
            Literal::BigInt(v) => {
                SBigInt::new(js_sys::BigInt::from_str(v.to_string().as_str())?)?.into()
            }
            Literal::SigmaProp(v) => SSigmaProp::from(*v).into(),
            Literal::GroupElement(v) => {
                SGroupElement::try_from((*v).sigma_serialize_bytes().map_err_js_value()?)?.into()
            }
            Literal::AvlTree(_) => todo!(),
            Literal::CBox(v) => {
                let native_box = &*v;
                let ergo_box = ErgoBox::from(native_box.clone());

                SErgoBox::new(&ergo_box).into()
            }
            Literal::Coll(v) => SColl::from(v).into(),
            Literal::Opt(_) => todo!(),
            Literal::Tup(_) => todo!(),
        };

        Ok(v.into())
    }

    /// Get the type of the {@link SConstant} instance.
    /// For example: "SColl(SByte)"
    #[wasm_bindgen(getter, js_name = typeStr)]
    pub fn tpe(&self) -> String {
        format!("{:?}", self.0.tpe)
    }

    /// Get a debug string representation of the {@link SConstant} instance.
    #[wasm_bindgen]
    pub fn dbg(&self) -> String {
        format!("{:?}", self.0)
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
    ErgoBox(Box<SErgoBox>),
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

impl From<SLiteral> for JsValue {
    fn from(value: SLiteral) -> Self {
        match value {
            SLiteral::Unit(v) => v.into(),
            SLiteral::Boolean(v) => v.into(),
            SLiteral::Byte(v) => v.into(),
            SLiteral::Short(v) => v.into(),
            SLiteral::Int(v) => v.into(),
            SLiteral::Long(v) => v.into(),
            SLiteral::BigInt(v) => v.into(),
            SLiteral::SigmaProp(v) => v.into(),
            SLiteral::GroupElement(v) => v.into(),
            SLiteral::ErgoBox(v) => (*v).into(),
            SLiteral::Coll(v) => v.into(),
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
            SLiteral::ErgoBox(v) => (*v).into(),
            SLiteral::Coll(v) => v.into(),
        }
    }
}

impl From<Literal> for SLiteral {
    fn from(v: Literal) -> Self {
        match v {
            Literal::Unit => SLiteral::Unit(SUnit(())),
            Literal::Boolean(b) => SLiteral::Boolean(b.into()),
            Literal::Byte(v) => SLiteral::Byte(v.into()),
            Literal::Short(v) => SLiteral::Short(v.into()),
            Literal::Int(v) => SLiteral::Int(v.into()),
            Literal::Long(v) => SLiteral::Long(v.into()),
            Literal::BigInt(v) => SLiteral::BigInt(v.into()),
            Literal::SigmaProp(v) => SLiteral::SigmaProp((*v).into()),
            Literal::GroupElement(v) => SLiteral::GroupElement((*v).into()),
            Literal::AvlTree(_) => todo!(),
            Literal::CBox(v) => SLiteral::ErgoBox(Box::new((*v).clone().into())),
            Literal::Coll(v) => SLiteral::Coll(v.into()),
            Literal::Opt(_) => todo!(),
            Literal::Tup(_) => todo!(),
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
        #[derive(Debug, Clone, From, Into)]
        pub struct $l($ty);

        #[wasm_bindgen]
        impl $l {
            #[wasm_bindgen(constructor)]
            pub fn new(value: $ty) -> Self {
                Self(value)
            }

            /// Convert the instance into it's JS value equivalent.
            #[wasm_bindgen(getter)]
            pub fn value(&self) -> $ty {
                self.0.clone()
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
pub struct SUnit(());

#[wasm_bindgen]
impl SUnit {
    /// Create a {@link SUnit} instance.
    #[wasm_bindgen(constructor)]
    pub fn new() -> SUnit {
        Self(())
    }

    /// Convert the instance into it's JS value equivalent.
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
#[derive(Debug, Clone, From, Into)]
pub struct SLong(i64);

#[wasm_bindgen]
impl SLong {
    /// Create a {@link SLong} from the provided {@link https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt BigInt}.
    ///
    /// We need to use {@external BigInt} because JavaScript numbers don't support the max value of i64.
    #[wasm_bindgen(constructor)]
    pub fn new(value: js_sys::BigInt) -> Result<SLong, JsValue> {
        Ok(Self(value.try_into().map_err_js_value()?))
    }

    /// Convert the instance into it's JS value equivalent.
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> js_sys::BigInt {
        js_sys::BigInt::from(self.0)
    }
}

impl From<SLong> for Literal {
    fn from(value: SLong) -> Self {
        value.0.into()
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone, From, Into)]
pub struct SBigInt(BigInt256);

#[wasm_bindgen]
impl SBigInt {
    /// Create a {@link SBigInt} from the provided {@link https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt BigInt}.
    #[wasm_bindgen(constructor)]
    pub fn new(js_value: js_sys::BigInt) -> Result<SBigInt, JsValue> {
        let radix = 10;
        let s = js_value
            .to_string(radix)
            .map_err_js_value()?
            .as_string()
            .ok_or_else(|| JsValue::from_str("failed to convert JS string"))?;
        let bi = BigInt256::from_str_radix(&s, radix as u32).map_err_js_value()?;

        Ok(SBigInt(bi))
    }

    /// Convert the instance into it's JS value equivalent.
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> Result<js_sys::BigInt, JsValue> {
        let s = self.0.to_string();

        js_sys::BigInt::from_str(s.as_str()).map_err_js_value()
    }
}

impl From<SBigInt> for Literal {
    fn from(bigint: SBigInt) -> Self {
        bigint.0.into()
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone, From, Into)]
pub struct SSigmaProp(SigmaProp);

#[wasm_bindgen]
impl SSigmaProp {
    /// Create a {@link SSigmaProp} instance from the provided JSON string.
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: &str) -> Result<SSigmaProp, JsValue> {
        let value: SigmaBoolean = serde_json::from_str(json).map_err_js_value()?;
        let prop: SigmaProp = value.into();

        Ok(prop.into())
    }

    /// Create a trivial {@link SSigmaProp} instance from the provided boolean value.
    #[wasm_bindgen(js_name = fromBool)]
    pub fn from_bool(bool: bool) -> SSigmaProp {
        SigmaProp::new(bool.into()).into()
    }

    /// Convert the instance into it's JS value equivalent.
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        match self.0.value() {
            SigmaBoolean::TrivialProp(bool) => (*bool).into(),
            SigmaBoolean::ProofOfKnowledge(_) => todo!(),
            SigmaBoolean::SigmaConjecture(_) => todo!(),
        }
    }
}

impl From<SSigmaProp> for Literal {
    fn from(prop: SSigmaProp) -> Self {
        prop.0.into()
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone, From, Into)]
pub struct SGroupElement(EcPoint);

#[wasm_bindgen]
impl SGroupElement {
    /// Create a {@link SGroupElement} from the provided hex string.
    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(hex: &str) -> Result<SGroupElement, JsValue> {
        let ec: EcPoint = hex.to_string().try_into().map_err_js_value()?;

        Ok(ec.into())
    }

    /// Create a {@link SGroupElement} from the provided byte array.
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &Uint8Array) -> Result<SGroupElement, JsValue> {
        let ec = EcPoint::sigma_parse_bytes(&bytes.to_vec()).map_err_js_value()?;

        Ok(ec.into())
    }

    /// Convert the instance into it's JS value equivalent.
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> Result<Uint8Array, JsValue> {
        let bytes = self.0.sigma_serialize_bytes().map_err_js_value()?;

        Ok(Uint8Array::from(bytes.as_slice()))
    }
}

impl From<SGroupElement> for Literal {
    fn from(prop: SGroupElement) -> Self {
        prop.0.into()
    }
}

impl TryFrom<Vec<u8>> for SGroupElement {
    type Error = JsValue;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let arr = Uint8Array::from(value.as_slice());

        Self::from_bytes(&arr)
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone, From, Into)]
pub struct SErgoBox(NativeErgoBox);

#[wasm_bindgen]
impl SErgoBox {
    #[wasm_bindgen(constructor)]
    pub fn new(ergo_box: &ErgoBox) -> SErgoBox {
        SErgoBox(ergo_box.clone().into())
    }

    /// Convert the instance into it's JS value equivalent.
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> ErgoBox {
        self.0.clone().into()
    }
}

impl From<SErgoBox> for Literal {
    fn from(prop: SErgoBox) -> Self {
        prop.0.into()
    }
}

impl TryFrom<&JsValue> for Box<SErgoBox> {
    type Error = JsValue;

    fn try_from(value: &JsValue) -> Result<Self, Self::Error> {
        let ergo_box = value.try_into()?;

        Ok(Box::new(ergo_box))
    }
}

#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct SColl(Vec<SLiteral>);

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

        Ok(SColl(coll))
    }

    #[wasm_bindgen(js_name = intoConstant)]
    pub fn into_constant(self) -> Result<SConstant, JsValue> {
        let inner: Constant = Value::from(Literal::from(self))
            .try_into()
            .map_err_js_value()?;

        Ok(SConstant(inner))
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> Array {
        let arr = Array::new();

        for v in self.0.iter() {
            let js_value = JsValue::from(v.clone());
            let literal = SLiteralLike::from(js_value);

            arr.push(&literal.get_value());
        }

        arr
    }
}

impl SColl {
    fn elem_tpe(&self) -> SType {
        // when creating a `SColl` we should check that it is valid
        // in the constructor so `unwrap()` is safe here
        self.0.get(0).unwrap().dyn_stype()
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
            .0
            .iter()
            .map(|v| v.clone().into())
            .collect::<Vec<Literal>>();

        Literal::Coll(CollKind::WrappedColl {
            elem_tpe: value.elem_tpe(),
            items,
        })
    }
}

impl From<CollKind<Literal>> for SColl {
    fn from(coll: CollKind<Literal>) -> Self {
        let values = coll
            .as_vec()
            .into_iter()
            .map(SLiteral::from)
            .collect::<Vec<_>>();

        Self(values)
    }
}

macro_rules! impl_literal_into_constant_method {
    ($($l:ident)*) => {$(
        #[wasm_bindgen]
        impl $l {
            /// Converts this literal into a {@link SConstant}.
            /// This method consumes the class instance so it should not be used after calling.
            #[wasm_bindgen(js_name = intoConstant)]
            pub fn into_constant(self) -> SConstant {
                let constant: Constant = self.0.into();

                constant.into()
            }
        }
    )*};
}

// Provide a method of `intoConstant` for each of the types with JS documentation
impl_literal_into_constant_method!(SUnit SBoolean SByte SShort SInt SLong SBigInt SSigmaProp SGroupElement SErgoBox);
