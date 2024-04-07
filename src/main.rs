/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0
*/
use clap::Parser;
use pwdg::DEFAULT_PWDGEN_OPTIONS as DEF;

#[derive(Parser)]
#[clap(about, version, author)]
struct Cli {
  /// Sets the length of the password. Must be at least 8.
  #[clap(short, long, default_value_t = pwdg::MIN_LENGTH)]
  length: usize,

  /// Minimum number of uppercase characters (A to Z).
  #[clap(long, default_value_t = DEF.min_upper)]
  min_upper: usize,

  /// Minimum number of lowercase characters (a to z).
  #[clap(long, default_value_t = DEF.min_lower)]
  min_lower: usize,

  /// Minimum number of digit characters (0 to 9).
  #[clap(long, default_value_t = DEF.min_digit)]
  min_digit: usize,

  /// Minimum number of special characters.
  #[clap(long, default_value_t = DEF.min_special, help = &format!(
    "Minimum number of special characters.\nSpecial characters: {}",
    pwdg::SPECIAL_CHARS.iter().collect::<String>()
  ))]
  min_special: usize,

  /// Characters to exclude from the overall character set used for password
  /// generation.
  #[clap(short, long)]
  exclude: Option<String>,

  /// Generates a password with at least 1 uppercase letter, 1 lowercase letter,
  /// 1 digit, and 1 special character. This option overrides --min-upper,
  /// --min-lower, --min-digit, and --min-special if they are also set.
  #[clap(short, long, action = clap::ArgAction::SetTrue)]
  strong: bool,
}

fn main() {
  let cli = Cli::parse();

  if let Err(e) = run(cli) {
    eprintln!("{}", e);
    std::process::exit(1);
  }
}

fn run(cli: Cli) -> Result<(), pwdg::Error> {
  let options = get_options(&cli)?;
  let password = pwdg::gen(cli.length, Some(options))?;

  println!("{}", password);

  Ok(())
}

fn get_options(cli: &Cli) -> Result<pwdg::PwdGenOptions, pwdg::Error> {
  let mut options = pwdg::PwdGenOptions::default();

  if cli.strong {
    options.min_upper = 1;
    options.min_lower = 1;
    options.min_digit = 1;
    options.min_special = 1;
  } else {
    options.min_upper = cli.min_upper;
    options.min_lower = cli.min_lower;
    options.min_digit = cli.min_digit;
    options.min_special = cli.min_special;
  }

  options.exclude = cli.exclude.as_deref();

  Ok(options)
}
