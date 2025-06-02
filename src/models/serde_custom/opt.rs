use serde::{Deserialize, Deserializer, Serializer};

use crate::cli::Opt;

pub fn serialize<S>(o: &Opt, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match o {
        Opt::String(o) => s.serialize_str(&format!("s{}", o)),
        Opt::Bool(b) => s.serialize_str(if *b { "b1" } else { "b0" }),
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
        o => Ok(Opt::String(o.trim_start_matches("s").to_string())),
    }
}
