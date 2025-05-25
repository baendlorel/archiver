use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(b: &bool, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u8(if *b { 1 } else { 0 })
}

pub fn deserialize<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let v = u8::deserialize(d)?;
    match v {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(serde::de::Error::custom("expected 0 or 1 for bool")),
    }
}
