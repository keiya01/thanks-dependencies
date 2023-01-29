mod fetcher;
mod readme;
mod registry;

use clap::Parser;
use readme::write_to_readme;
use std::{
    fs::OpenOptions,
    io::{Read, Result, Seek, SeekFrom, Write},
    path::PathBuf,
};

/// Write thanks dependencies list on README.
/// This has process to connect cargo.io to fetch dependency information.
#[derive(Parser)]
struct Cli {
    /// Path for your README.md.
    #[arg(short, long, default_value = "./README.md")]
    readme: PathBuf,
    /// Path for your Cargo.toml.
    #[arg(short, long, default_value = "./Cargo.toml")]
    cargo: PathBuf,
    /// Title for thanks list.
    #[arg(
        short,
        long,
        default_value = "## Thanks for the following dependencies"
    )]
    title: String,
    /// TODO: Not implemented yet. Whether reading dependencies recursively.
    #[arg(long, default_value_t = false)]
    recursive: bool,
}

fn main() -> Result<()> {
    let c = Cli::parse();

    let mut f = OpenOptions::new().read(true).write(true).open(c.readme)?;
    let mut readme = String::new();
    f.read_to_string(&mut readme)?;

    f.seek(SeekFrom::Start(0)).unwrap();

    let content = futures::executor::block_on(registry::read_content(c.cargo));
    let content = write_to_readme(readme, c.title, content);
    f.write_all(content.as_bytes())
        .expect("Failed to write content to readme");

    Ok(())
}
