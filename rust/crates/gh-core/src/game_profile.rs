//! Game identity and provider boundaries for the HSR fork.
//!
//! This module intentionally contains no capture or CV logic.  It is the
//! stable boundary used by the app host to decide which game window it may
//! observe and which map provider route belongs to that game.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameProfile {
    pub id: &'static str,
    pub display_name: &'static str,
    pub window_titles: &'static [&'static str],
    pub hoyolab_map_url: &'static str,
}
pub const HSR_GAME_ID: &str = "honkai-star-rail";
pub const HSR_DISPLAY_NAME: &str = "Honkai: Star Rail Helper";
pub const HSR_WINDOW_TITLES: [&str; 3] = [
    "Honkai: Star Rail",
    "崩坏：星穹铁道",
    "崩壊：スターレイル",
];
pub const HSR_HOYOLAB_MAP_URL: &str = "https://act.hoyolab.com/sr/app/interactive-map/";
pub const HSR_HOYOLAB_API_BASE: &str =
    "https://sg-act-public-api-static.hoyolab.com/common/srmap/sr_map";
pub const HSR_HOYOLAB_API_NO_CDN_BASE: &str =
    "https://sg-act-public-api.hoyolab.com/common/srmap/sr_map";
pub const HSR_HOYOLAB_APP_SIGN: &str = "sr_map";
pub const HSR_HOYOLAB_APP_VERSION: &str = "de16a09fca4e0ab89acf69fe0c12514f";
/// Kept false until an HSR-specific minimap calibration is available.
/// Reusing the Teyvat tracker would create plausible-looking but wrong poses.
pub const HSR_MINIMAP_TRACKING_SUPPORTED: bool = false;
pub const HSR_MAP_IDS: [u32; 17] = [
    38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
];

pub const HSR_PROFILE: GameProfile = GameProfile {
    id: HSR_GAME_ID,
    display_name: HSR_DISPLAY_NAME,
    window_titles: &HSR_WINDOW_TITLES,
    hoyolab_map_url: HSR_HOYOLAB_MAP_URL,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsr_profile_has_only_hsr_identity() {
        assert_eq!(HSR_PROFILE.id, "honkai-star-rail");
        assert_eq!(HSR_PROFILE.window_titles.len(), 3);
        assert!(HSR_PROFILE.window_titles.contains(&"Honkai: Star Rail"));
        assert!(HSR_PROFILE.hoyolab_map_url.starts_with("https://act.hoyolab.com/sr/"));
        assert_eq!(HSR_HOYOLAB_APP_SIGN, "sr_map");
        assert!(HSR_MAP_IDS.contains(&38));
        assert!(HSR_MAP_IDS.contains(&54));
    }
}
