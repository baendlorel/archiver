use serde::{Serialize, de::DeserializeOwned};

pub trait JsonSerde: Sized + Serialize + DeserializeOwned {
    /// 从json字符串转换为对象
    fn from_json_string(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }

    /// 把json转换为好看格式的字符串
    fn to_formatted_string(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    /// 把json转换为一行的字符串
    fn to_json_line(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

// blanket impl for all Serialize + DeserializeOwned types
impl<T> JsonSerde for T where T: Sized + Serialize + DeserializeOwned {}
