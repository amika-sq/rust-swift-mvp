use crate::UniffiCustomTypeConverter;
use ssi_jwk::JWK;

pub struct Jwk(pub JWK);

uniffi::custom_type!(Jwk, String);

impl UniffiCustomTypeConverter for Jwk {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        let jwk: JWK = serde_json::from_str(&val).expect("whoops");
        Ok(Jwk(jwk))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        serde_json::to_string(&obj.0).expect("oh noes")
    }
}
