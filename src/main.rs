use std::fs;
use std::path::PathBuf;
use std::str::FromStr as _;

use anyhow::Result;
use onlyargs_derive::OnlyArgs;
use regexer::RegexCommands;

// Don't need a Args rustdoc here because our onlyargs scrapes from the Cargo.toml description I guess??
#[derive(OnlyArgs)]
struct Args {
    /// The source file to run regex replacements on.
    file: PathBuf,
    /// The file of regex replacements.
    replacements: PathBuf,
    /// Output file name, if desired.
    out: Option<PathBuf>,
}

fn main() -> Result<()> {
    simple_logger::SimpleLogger::new().init().unwrap();

    let config: Args = onlyargs::parse()?;

    let source = fs::read_to_string(&config.file)?;
    let replacement_commands = RegexCommands::from_str(&fs::read_to_string(config.replacements)?)?;
    let out_text = replacement_commands.transmorgify(&source);
    let dest_file: String = config
        .out
        .map(|pbuf| pbuf.to_string_lossy().to_string())
        .unwrap_or(config.file.to_string_lossy().to_string() + ".replaced");
    fs::write(dest_file, out_text)?;

    Ok(())
}
