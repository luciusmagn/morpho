use std::path::{Path, PathBuf};

use log::{debug, info};
use tera::Tera;

use crate::error::{Error, Result};
use crate::utils::{read_file, write_file};

macro_rules! try_init_template {
	($render:expr, $tpl_name:expr, $tpl_str:expr) => {
		let template_content = match std::str::from_utf8(&$tpl_str) {
			Ok(content) => content,
			Err(_) => return Err(Error::ThemeFileEncoding($tpl_name.into())),
			};
		$render.add_raw_template($tpl_name, template_content)?;
	};
}

/// blog theme object
#[derive(Default, Debug)]
pub struct Theme {
	/// theme root directory
	pub root:     PathBuf,
	/// theme name
	pub name:     String,
	/// theme renderer
	pub renderer: Tera,
	favicon:      Vec<u8>,
	logo:         Vec<u8>,
	feed:         Vec<u8>,
	main_css:     Vec<u8>,
	main_js:      Vec<u8>,
	base:         Vec<u8>,
	index:        Vec<u8>,
	post:         Vec<u8>,
	tag:          Vec<u8>,
}

impl Theme {
	pub fn new<P: AsRef<Path>>(root: P, name: &str) -> Result<Theme> {
		debug!("loading theme: {}", &name);
		let root = root.as_ref();
		let mut theme = Theme {
			root: root.to_owned(),
			name: name.to_string(),
			renderer: Tera::default(),
			..Default::default()
		};
		let src_dir = root.join(name);
		if !src_dir.exists() {
			if name != "simple" {
				return Err(Error::ThemeNotFound(name.into()));
			}
			theme.favicon.extend_from_slice(&SIMPLE_FAVICON);
			theme.logo.extend_from_slice(&SIMPLE_LOGO);
			theme.feed.extend_from_slice(&SIMPLE_FEED);
			theme.main_css.extend_from_slice(&SIMPLE_MAIN_CSS);
			theme.main_js.extend_from_slice(&SIMPLE_MAIN_JS);
			theme.base.extend_from_slice(&SIMPLE_BASE);
			theme.index.extend_from_slice(&SIMPLE_INDEX);
			theme.post.extend_from_slice(&SIMPLE_POST);
			theme.tag.extend_from_slice(&SIMPLE_TAG);
			theme.init_template()?;
			return Ok(theme);
		}

		read_file(&src_dir.join("static/favicon.png"), &mut theme.favicon)?;
		read_file(&src_dir.join("static/main.css"), &mut theme.main_css)?;
		read_file(&src_dir.join("static/main.js"), &mut theme.main_js)?;
		read_file(&src_dir.join("templates/base.tpl"), &mut theme.base)?;
		read_file(&src_dir.join("templates/index.tpl"), &mut theme.index)?;
		read_file(&src_dir.join("templates/post.tpl"), &mut theme.post)?;
		read_file(&src_dir.join("templates/tag.tpl"), &mut theme.tag)?;
		theme.init_template()?;
		return Ok(theme);
	}

	/// init renderer template.
	fn init_template(&mut self) -> Result<()> {
		try_init_template!(self.renderer, "base.tpl", self.base);
		try_init_template!(self.renderer, "index.tpl", self.index);
		try_init_template!(self.renderer, "post.tpl", self.post);
		try_init_template!(self.renderer, "tag.tpl", self.tag);
		Ok(())
	}

	/// create theme directory.
	pub fn init_dir(&self, name: &str) -> Result<()> {
		let dest_dir = self.root.join(name);
		if dest_dir.exists() {
			info!("theme({}) already existed", name);
			return Ok(());
		}
		debug!("init theme({}) ...", name);
		write_file(&dest_dir.join("static/favicon.png"), &self.favicon)?;
		write_file(&dest_dir.join("static/logo.png"), &self.logo)?;
		write_file(&dest_dir.join("static/feed.png"), &self.feed)?;
		write_file(&dest_dir.join("static/main.css"), &self.main_css)?;
		write_file(&dest_dir.join("static/main.js"), &self.main_js)?;
		write_file(&dest_dir.join("templates/base.tpl"), &self.base)?;
		write_file(&dest_dir.join("templates/index.tpl"), &self.index)?;
		write_file(&dest_dir.join("templates/post.tpl"), &self.post)?;
		write_file(&dest_dir.join("templates/tag.tpl"), &self.tag)?;
		Ok(())
	}

	/// export theme static files.
	pub fn export_static<P: AsRef<Path>>(&self, root: P) -> Result<()> {
		debug!("exporting theme({}) static ...", self.name);
		let dest_dir = root.as_ref();
		write_file(&dest_dir.join("static/favicon.png"), &self.favicon)?;
		write_file(&dest_dir.join("static/logo.png"), &self.logo)?;
		write_file(&dest_dir.join("static/feed.png"), &self.feed)?;
		write_file(&dest_dir.join("static/main.css"), &self.main_css)?;
		write_file(&dest_dir.join("static/main.js"), &self.main_js)?;
		Ok(())
	}
}

static SIMPLE_FAVICON: &'static [u8] = include_bytes!("simple_theme/static/favicon.png");
static SIMPLE_LOGO: &'static [u8] = include_bytes!("simple_theme/static/favicon.png");
static SIMPLE_FEED: &'static [u8] = include_bytes!("simple_theme/static/favicon.png");
static SIMPLE_MAIN_CSS: &'static [u8] = include_bytes!("simple_theme/static/main.css");
static SIMPLE_MAIN_JS: &'static [u8] = include_bytes!("simple_theme/static/main.js");
static SIMPLE_BASE: &'static [u8] = include_bytes!("simple_theme/templates/base.tpl");
static SIMPLE_INDEX: &'static [u8] = include_bytes!("simple_theme/templates/index.tpl");
static SIMPLE_POST: &'static [u8] = include_bytes!("simple_theme/templates/post.tpl");
static SIMPLE_TAG: &'static [u8] = include_bytes!("simple_theme/templates/tag.tpl");
