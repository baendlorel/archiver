use crate::wrap_err;

use std::fs;

use crate::{
    misc::paths,
    models::{error::ArchiverError, types::ArchiverConfig},
};

pub fn load() -> Result<ArchiverConfig, ArchiverError> {
    // 在设置全局变量时已经创建了假如不存在的config.json
    let config_path = paths::CONFIG_FILE_PATH.clone();
    let content = wrap_err!(fs::read_to_string(config_path))?;
    Ok(wrap_err!(serde_json::from_str::<ArchiverConfig>(&content))?)
}

pub fn save(config: &ArchiverConfig) -> Result<(), ArchiverError> {
    let json_str = wrap_err!(serde_json::to_string_pretty(config))?;
    wrap_err!(fs::write(paths::CONFIG_FILE_PATH.clone(), json_str))?;
    Ok(())
}
