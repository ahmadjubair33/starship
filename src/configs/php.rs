use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct PhpConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for PhpConfig<'a> {
    fn default() -> Self {
        PhpConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "🐘 ",
            style: "147 bold",
            disabled: false,
            detect_extensions: vec!["php"],
            detect_files: vec!["composer.json", ".php-version"],
            detect_folders: vec![],
        }
    }
}
