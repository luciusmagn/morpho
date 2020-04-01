use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

use log::error;
use morpho::{Mdblog, Result};
use argh::FromArgs;

#[derive(FromArgs, Debug, PartialEq)]
#[argh(subcommand, name = "init")]
/// initialize the blog directory layout
struct Init {
	#[argh(positional)]
	/// the blog directory name
	name: String,
}

#[derive(FromArgs, Debug, PartialEq)]
#[argh(subcommand, name = "new")]
/// create a blog post
struct New {
	#[argh(option, short = 't', long = "tag")]
	/// post tags
	tags: Vec<String>,
	#[argh(positional)]
	/// post path relative to blog `posts` directory
	path: PathBuf
}

//#[derive(FromArgs, Debug, PartialEq)]
//#[argh(subcommand, name = "build")]
//  build the blog static files
//struct Build;

#[derive(FromArgs, Debug, PartialEq)]
#[argh(subcommand, name = "serve")]
/// serve the blog at <port>
struct Serve {
	#[argh(option, short = 'p', long = "port", default = "5000")]
	/// the port to use
	port: u16
}

#[derive(FromArgs, Debug, PartialEq)]
#[argh(subcommand, name = "build")]
/// build the site
struct Build {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Commands {
	/// initialize the blog directory layout
	Init(Init),
	/// create a blog post
	New(New),
	/// build the blog static files
	Build(Build),
	/// serve the blog, rebuild on change
	Serve(Serve),
}

#[derive(FromArgs, PartialEq, Debug)]
/// static site generator from markdown files
struct Opt {
    #[argh(subcommand)]
    command: Commands,
}

fn main() {
	env_logger::Builder::from_default_env().filter(None, log::LevelFilter::Info).init();
	let opt: Opt = argh::from_env();
	let res = match opt.command {
		Commands::Init(init_cmd) => init(&init_cmd.name),
		Commands::New(new_cmd) => new(&new_cmd.path, &new_cmd.tags),
		Commands::Build(_) => build(),
		Commands::Serve(serve_cmd) => serve(serve_cmd.port),
	};

	if let Err(ref e) = res {
		log_error_chain(e);
		std::process::exit(1);
	}
}

fn init(name: &str) -> Result<()> {
	let root_dir = env::current_dir()?.join(name);
	let mut mb = Mdblog::new(root_dir)?;
	mb.init()?;
	Ok(())
}

fn new(path: &Path, tags: &[String]) -> Result<()> {
	let root_dir = env::current_dir()?;
	let mut mb = Mdblog::new(&root_dir)?;
	mb.load_customize_settings()?;
	mb.create_post(path, tags)?;
	Ok(())
}

fn build() -> Result<()> {
	let root_dir = env::current_dir()?;
	let mut mb = Mdblog::new(&root_dir)?;
	mb.load_customize_settings()?;
	mb.build()?;
	Ok(())
}

fn serve(port: u16) -> Result<()> {
	let root_dir = env::current_dir()?;
	let mut mb = Mdblog::new(&root_dir)?;
	mb.load_customize_settings()?;
	mb.serve(port)?;
	Ok(())
}

fn log_error_chain(mut e: &dyn Error) {
	error! {"error debug: {:?}", e};
	error!("error: {}", e);
	while let Some(source) = e.source() {
		error!("caused by: {}", source);
		e = source;
	}
}
