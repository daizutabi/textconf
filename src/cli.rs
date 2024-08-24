use anyhow::Context;
use clap::Parser;
use std::{env, fs, path::PathBuf};

use crate::settings::Settings;

/// Generate a config file from text for Python's Hydra applications"
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// A space separated list of file, directory or glob
    #[arg(required_unless_present = "stdin")]
    input_patterns: Option<Vec<String>>,

    /// Read stdin and write to stdout
    #[arg(short, long, default_value = "false")]
    stdin: bool,

    /// Maximum width of each line
    #[arg(short, long)]
    max_width: Option<usize>,

    /// Configuration file
    #[arg(short, long)]
    config_file: Option<PathBuf>,

    #[arg(
        short,
        long,
        default_value = "false",
        default_value_if("stdin", "true", "true")
    )]
    quiet: bool,

    /// Check if the file is correctly formatted. Exit with code 1 if not.
    #[arg(long, default_value = "false")]
    check: bool,
}

pub(crate) fn run() {
    let args = Args::parse();
    let settings = Settings::from(&args).unwrap();
    let quiet = args.quiet;

    // Print settings
    if !quiet {
        println!("{}", toml::to_string_pretty(&settings).unwrap());
    }
}

fn find_config_file() -> anyhow::Result<Option<PathBuf>> {
    Ok(dunce::canonicalize(env::current_dir()?)?
        .ancestors()
        .map(|p| p.join("textconf.toml"))
        .find(|p| p.exists()))
}

fn load_config(path: &PathBuf) -> anyhow::Result<Settings> {
    fs::read_to_string(path)
        .context("could not read config file")
        .and_then(|contents| toml::from_str(&contents).context("could not parse config file"))
        .with_context(|| format!("failed to load config file: {}", path.display()))
}

impl Settings {
    fn from(args: &Args) -> anyhow::Result<Self> {
        let mut settings = args
            .config_file
            .as_ref()
            .map(load_config)
            .or_else(|| {
                find_config_file()
                    .and_then(|v| v.as_ref().map(load_config).transpose())
                    .transpose()
            })
            .transpose()?
            .unwrap_or_default();

        if let Some(max_width) = args.max_width {
            settings.max_width = max_width;
        }

        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_find_config_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("textconf.toml");
        let mut file = std::fs::File::create(&config_path).unwrap();
        writeln!(file, "max_width = 120").unwrap();

        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = find_config_file().unwrap();
        assert_eq!(result, Some(config_path));
    }

    #[test]
    fn test_load_config() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "max_width = 120").unwrap();

        let settings = load_config(&temp_file.path().to_path_buf()).unwrap();
        assert_eq!(settings.max_width, 120);
    }
}
