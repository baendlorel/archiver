use crate::must_ok;

use once_cell::sync::Lazy;

use crate::misc::paths;
use crate::models::types::ArchiverConfig;
use crate::traits::CustomColors;

pub static CONFIG: Lazy<ArchiverConfig> = Lazy::new(|| {
    // 在设置全局变量时已经创建了假如不存在的config.json
    let content = must_ok!(
        std::fs::read_to_string(paths::CONFIG_FILE_PATH.as_path()),
        "Fail to read config file"
    );
    let mut config = must_ok!(
        serde_json::from_str::<ArchiverConfig>(&content),
        "Fail to parse config file"
    );

    // 下面进行一些正规化

    // 保持这个开关不是on就是off
    if config.update_check != "on" {
        config.update_check = "off".to_string();
    }

    // alias_map增加默认的~路径
    config.alias_map.insert(
        "~".to_string(),
        paths::HOME_DIR.to_string_lossy().to_string(),
    );

    config
});

/// vault item的分隔符，默认为冒号
/// - 如果vault名为`@`，归档项名字为`temp`，则输出格式为`@:temp`
/// - 会影响
///     - ListEntry的显示
///     - LogEntry的显示
pub static VLT_ITEM_SEP: Lazy<String> =
    Lazy::new(|| CONFIG.vault_item_sep.styled_vault_item_seperator());
