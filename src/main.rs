use std::{fmt, fs, path::PathBuf, str::FromStr, sync::LazyLock};

use anyhow::{Context, Error, Result, anyhow};
use onlyargs_derive::OnlyArgs;
use regex::Regex;

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

struct RegexCommand {
    find: String,
    find_compiled: Regex,
    replace: String,
}

impl fmt::Debug for RegexCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RegexCommand {{ find: \"{}\", replace: \"{}\" }}",
            self.find, self.replace
        )
    }
}

/// Regex to parse a RegexCommand from a line of text.
static COMMAND_FROM_LINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""(.*)"\s*?->\s*?"(.*)""#).unwrap());

impl RegexCommand {
    fn new(search: String, replace: String) -> Result<Self> {
        let search_compiled = Regex::new(&search)?;
        Ok(Self {
            find: search,
            find_compiled: search_compiled,
            replace,
        })
    }

    fn from_line(line: &str) -> Result<Self> {
        assert!(!line.contains("\n") && !line.contains("\r\n"));

        let (_, [search, replace]) = COMMAND_FROM_LINE
            .captures(line)
            .context("could not parse line. regex failed to find.")?
            .extract();

        // program in escape sequences
        let replace = replace.replace("\\n", "\n");
        let replace = replace.replace("\\r", "\r");
        let replace = replace.replace("\\\\", "\\");

        Self::new(search.to_string(), replace)
    }

    /// Transmorgify subject matter using our stored regex command.
    fn transmorgify(&self, subject: &str) -> String {
        self.find_compiled
            .replace_all(subject, &self.replace)
            .to_string()
    }
}

/// A series of find and replace commands.
#[derive(Default)]
struct RegexCommands(Vec<RegexCommand>);

impl RegexCommands {
    /// Transmorgify subject matter using our series of stored regex commands.
    fn transmorgify(&self, subject: &String) -> String {
        let mut subject = subject.to_owned();
        for command in &self.0 {
            subject = command.transmorgify(&subject);
        }
        subject.replace("\r\n", "\n").replace("\u{00A0}", " ")
    }
}

impl FromStr for RegexCommands {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut commands = RegexCommands(vec![]);

        for (i, line) in s.replace("\r\n", "\n").split("\n").enumerate() {
            if line.is_empty() {
                log::trace!("line {i} IGNORED");
                continue;
            }

            let command = RegexCommand::from_line(line)
                .context(anyhow!("failure on line {i} of replacements file"))?;
            log::trace!("line {i} parsed a {command:?}");
            commands.0.push(command);
        }

        Ok(commands)
    }
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
