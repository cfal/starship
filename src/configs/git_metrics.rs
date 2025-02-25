use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct GitMetricsConfig<'a> {
    pub added_style: &'a str,
    pub deleted_style: &'a str,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> Default for GitMetricsConfig<'a> {
    fn default() -> Self {
        GitMetricsConfig {
            added_style: "bold green",
            deleted_style: "bold red",
            format: "[+$added]($added_style) [-$deleted]($deleted_style) ",
            disabled: true,
        }
    }
}
