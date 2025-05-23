use crate::wrap_err_fatal;

use std::fs;

use crate::{
    misc::paths,
    models::{error::ArchiverError, json_serde::JsonSerde, types::ArchiverConfig},
};

pub fn load() -> Result<ArchiverConfig, ArchiverError> {
    // 在设置全局变量时已经创建了假如不存在的config.json
    let config_path = paths::CONFIG_FILE_PATH.clone();
    let content = wrap_err_fatal!(fs::read_to_string(config_path))?;
    let mut config = wrap_err_fatal!(serde_json::from_str::<ArchiverConfig>(&content))?;

    // 下面进行一些正规化
    // 保持这个开关不是on就是off
    if config.auto_check_update != "on" {
        config.auto_check_update = "off".to_string();
    }

    Ok(config)
}

pub fn save(config: &ArchiverConfig) -> Result<(), ArchiverError> {
    let json = wrap_err_fatal!(config.to_json_string())?;
    wrap_err_fatal!(fs::write(paths::CONFIG_FILE_PATH.as_path(), json))?;
    Ok(())
}
