use std::fmt;
use std::str::FromStr;
use std::sync::LazyLock;

use anyhow::{Context, Error, Result, anyhow};
use regex::Regex;

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
    LazyLock::new(|| Regex::new(r#"[^"]*"(.*)"\s*?->\s*?"(.*)"[^"]*"#).unwrap());

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
            .context("could not parse line.")?
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
pub struct RegexCommands(Vec<RegexCommand>);

impl RegexCommands {
    /// Transmorgify subject matter using our series of stored regex commands.
    pub fn transmorgify(&self, subject: &String) -> String {
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
            let i = i + 1; // editors usually 1-index their rows

            if line.is_empty() || line.starts_with("//") {
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
