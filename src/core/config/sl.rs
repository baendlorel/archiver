use crate::as_fatal;

use std::fs;

use crate::{
    misc::paths,
    models::{error::ArchiverError, serde_json::SerdeJson, types::ArchiverConfig},
};

pub fn save(config: &ArchiverConfig) -> Result<(), ArchiverError> {
    let json = as_fatal!(config.to_formatted_string())?;
    as_fatal!(fs::write(paths::CONFIG_FILE_PATH.as_path(), json))?;
    Ok(())
}

pub fn load() -> Result<ArchiverConfig, ArchiverError> {
    // 在设置全局变量时已经创建了假如不存在的config.json
    let config_path = paths::CONFIG_FILE_PATH.as_path();
    let content = as_fatal!(fs::read_to_string(config_path))?;
    let mut config = as_fatal!(serde_json::from_str::<ArchiverConfig>(&content))?;

    // 下面进行一些正规化
    // 保持这个开关不是on就是off
    if config.auto_check_update != "on" {
        config.auto_check_update = "off".to_string();
    }

    Ok(config)
}
