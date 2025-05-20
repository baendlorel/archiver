use crate::wrap_result;

use super::config_data;
use crate::models::error::ArchiverError;

pub fn show(config_item: &Option<String>) -> Result<(), ArchiverError> {
    let config = wrap_result!(config_data::load())?;

    // 制定了具体打印哪个配置
    if let Some(config_item) = config_item {
        println!("{}", config.show(config_item));
        return Ok(());
    }

    // 未指定，打印所有配置
    for item in config.get_items() {
        println!("{}", config.show(item));
    }

    Ok(())
}
