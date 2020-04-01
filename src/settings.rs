use serde::{Deserialize, Serialize};

#[rustfmt::skip]
pub(super)mod defaults {
    #[inline] pub(super) fn site_url()         -> String { "".into() }
    #[inline] pub(super) fn site_name()        -> String { "My site".into() }
    #[inline] pub(super) fn site_motto()       -> String { "Taxation is theft".into() }
    #[inline] pub(super) fn footer_note()      -> String { "Trump 2020".into() }
    #[inline] pub(super) fn media_dir()        -> String { "media".into() }
    #[inline] pub(super) fn build_dir()        -> String { "public".into() }
    #[inline] pub(super) fn theme()            -> String { "simple".into() }
    #[inline] pub(super) fn theme_root_dir()   -> String { "themes".into() }
    #[inline] pub(super) fn rebuild_interval() -> u8     { 2 }
    #[inline] pub(super) fn posts_per_page()   -> usize  { 999 }
}

/// site setting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
	/// site base url
	#[serde(default = "defaults::site_url")]
	pub site_url:         String,
	/// site site name
	#[serde(default = "defaults::site_name")]
	pub site_name:        String,
	/// site site motto
	#[serde(default = "defaults::site_motto")]
	pub site_motto:       String,
	/// site footer note
	#[serde(default = "defaults::footer_note")]
	pub footer_note:      String,
	/// site media directory
	#[serde(default = "defaults::media_dir")]
	pub media_dir:        String,
	/// site build root directory
	#[serde(default = "defaults::build_dir")]
	pub build_dir:        String,
	/// site theme name
	#[serde(default = "defaults::theme")]
	pub theme:            String,
	/// site theme root directory
	#[serde(default = "defaults::theme_root_dir")]
	pub theme_root_dir:   String,
	/// site rebuild interval
	#[serde(default = "defaults::rebuild_interval")]
	pub rebuild_interval: u8,
	/// post count per index page
	#[serde(default = "defaults::posts_per_page")]
	pub posts_per_page:   usize,
}

impl Default for Settings {
	fn default() -> Self {
		return Settings {
			site_url:         defaults::site_url(),
			site_name:        defaults::site_name(),
			site_motto:       defaults::site_motto(),
			footer_note:      defaults::footer_note(),
			media_dir:        defaults::media_dir(),
			build_dir:        defaults::build_dir(),
			theme:            defaults::theme(),
			theme_root_dir:   defaults::theme_root_dir(),
			rebuild_interval: defaults::rebuild_interval(),
			posts_per_page:   defaults::posts_per_page(),
		};
	}
}

