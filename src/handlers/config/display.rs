use crate::wrap_result;

use owo_colors::OwoColorize;

use super::sl;
use crate::misc::{ForceToString, paths};
use crate::models::error::ArchiverError;
use crate::models::types::CONFIG_ITEMS;

pub fn display(config_item: &Option<String>) -> Result<(), ArchiverError> {
    let config = wrap_result!(sl::load())?;

    // 制定了具体打印哪个配置
    if let Some(config_item) = config_item {
        println!("{}", config.display(config_item));
        return Ok(());
    }

    // 未指定，打印所有配置，并显示.archiver的路径
    let head = "Archiver Path ".fg_rgb::<153, 153, 153>();
    println!(
        "{}{}\n  {}",
        head,
        "(Cannot be modified)".cyan(),
        paths::ROOT_DIR.force_to_string()
    );

    for item in CONFIG_ITEMS {
        println!("{}", config.display(item));
    }

    Ok(())
}
