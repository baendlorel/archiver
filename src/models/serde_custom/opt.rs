use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

use crate::cli::Opt;

pub fn serialize<S>(o: &Opt, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match o {
        Opt::String(o) => s.serialize_str(&format!("s{}", o)),
        Opt::Bool(b) => s.serialize_str(if *b { "b1" } else { "b0" }),
        Opt::U32(u) => s.serialize_str(&format!("u32{}", u)),
    }
}

pub fn deserialize<'a, D>(d: D) -> Result<Opt, D::Error>
where
    D: Deserializer<'a>,
{
    let o: String = String::deserialize(d)?;
    match o.as_str() {
        "b1" => Ok(Opt::Bool(true)),
        "b0" => Ok(Opt::Bool(false)),
        o => {
            if o.starts_with("s") {
                return Ok(Opt::String(o.trim_start_matches("s").to_string()));
            }

            if o.starts_with("u32") {
                let u = o
                    .trim_start_matches("u32")
                    .parse::<u32>()
                    .map_err(D::Error::custom)?;
                return Ok(Opt::U32(u));
            }

            Ok(Opt::String(o.to_string())) // 默认返回字符串
        }
    }
}
