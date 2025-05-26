use crate::as_fatal;

use std::fs;

use crate::misc::paths;
use crate::models::{error::ArchiverResult, serde_custom::SerdeJson, types::ArchiverConfig};

pub fn save(config: &ArchiverConfig) -> ArchiverResult<()> {
    let json = as_fatal!(config.to_formatted_string())?;
    as_fatal!(fs::write(paths::CONFIG_FILE_PATH.as_path(), json))?;
    Ok(())
}

pub fn load() -> ArchiverResult<ArchiverConfig> {
    // 在设置全局变量时已经创建了假如不存在的config.json
    let content = as_fatal!(fs::read_to_string(paths::CONFIG_FILE_PATH.as_path()))?;
    let mut config = as_fatal!(serde_json::from_str::<ArchiverConfig>(&content))?;

    // 下面进行一些正规化
    // 保持这个开关不是on就是off
    if config.auto_check_update != "on" {
        config.auto_check_update = "off".to_string();
    }

    Ok(config)
}
