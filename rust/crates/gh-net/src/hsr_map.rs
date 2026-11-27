//! HoYoLAB Interactive Map contract for Honkai: Star Rail.
//!
//! This adapter is intentionally separate from `hoyolab_sync`.  That module
//! implements the legacy Genshin `ys_obc` contract and its data model cannot
//! be reused for Star Rail without silently producing wrong map coordinates.

use gh_core::game_profile::{
    HSR_HOYOLAB_API_BASE, HSR_HOYOLAB_APP_SIGN, HSR_HOYOLAB_APP_VERSION, HSR_MAP_IDS,
};
use reqwest::Url;

const DEFAULT_LANGUAGE: &str = "en-us";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HsrMapClientConfig {
    pub api_base: String,
    pub language: String,
    pub app_version: String,
}

impl Default for HsrMapClientConfig {
    fn default() -> Self {
        Self {
            api_base: HSR_HOYOLAB_API_BASE.to_owned(),
            language: DEFAULT_LANGUAGE.to_owned(),
            app_version: HSR_HOYOLAB_APP_VERSION.to_owned(),
        }
    }
}

impl HsrMapClientConfig {
    pub fn is_supported_map_id(map_id: u32) -> bool {
        HSR_MAP_IDS.contains(&map_id)
    }

    pub fn map_info_url(&self, map_id: u32) -> Result<Url, String> {
        self.endpoint("/v1/map/info", map_id, None)
    }

    pub fn point_list_url(&self, map_id: u32) -> Result<Url, String> {
        self.endpoint("/v1/map/point/list", map_id, None)
    }

    fn endpoint(
        &self,
        path: &str,
        map_id: u32,
        point_id: Option<u64>,
    ) -> Result<Url, String> {
        let base = format!("{}{}", self.api_base.trim_end_matches('/'), path);
        let mut query = vec![
            ("map_id", map_id.to_string()),
            ("app_sn", HSR_HOYOLAB_APP_SIGN.to_owned()),
            ("lang", self.language.clone()),
            ("app_version", self.app_version.clone()),
        ];
        if let Some(point_id) = point_id {
            query.push(("point_id", point_id.to_string()));
        }
        Url::parse_with_params(&base, query).map_err(|error| error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_targets_the_hsr_map_api() {
        let config = HsrMapClientConfig::default();
        assert_eq!(config.api_base, HSR_HOYOLAB_API_BASE);
        assert!(HsrMapClientConfig::is_supported_map_id(38));
        assert!(HsrMapClientConfig::is_supported_map_id(54));
        assert!(!HsrMapClientConfig::is_supported_map_id(2));
    }

    #[test]
    fn point_list_url_uses_hsr_contract() {
        let url = HsrMapClientConfig::default()
            .point_list_url(38)
            .expect("valid official endpoint");
        assert_eq!(
            url.as_str(),
            "https://sg-act-public-api-static.hoyolab.com/common/srmap/sr_map/v1/map/point/list?map_id=38&app_sn=sr_map&lang=en-us&app_version=de16a09fca4e0ab89acf69fe0c12514f"
        );
    }

    #[test]
    fn language_and_base_are_encoded_without_string_concatenation() {
        let config = HsrMapClientConfig {
            api_base: "https://example.test/api/".to_owned(),
            language: "vi-vn&unexpected=true".to_owned(),
            app_version: "test version".to_owned(),
        };
        let url = config.map_info_url(54).expect("valid encoded endpoint");
        assert!(url.as_str().contains("lang=vi-vn%26unexpected%3Dtrue"));
        assert!(url.as_str().contains("app_version=test+version"));
    }
}
