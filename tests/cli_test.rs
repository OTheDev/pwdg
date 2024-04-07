/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0
*/
use pwdg::SPECIAL_CHARS;
use std::process::Command;

fn run_app(args: &[&str]) -> Result<String, String> {
  let path = if cfg!(debug_assertions) {
    "./target/debug/pwdg"
  } else {
    "./target/release/pwdg"
  };

  let output = Command::new(path)
    .args(args)
    .output()
    .expect("failed to execute process");

  if output.status.success() {
    Ok(
      String::from_utf8(output.stdout)
        .unwrap_or_else(|_| "Non-UTF-8 output from command".to_string()),
    )
  } else {
    Err(
      String::from_utf8(output.stderr)
        .unwrap_or_else(|_| "Non-UTF-8 error output from command".to_string()),
    )
  }
}

#[test]
fn test_password_default_length() {
  if let Ok(output) = run_app(&[]) {
    assert_eq!(output.trim().len(), 8);
  } else {
    panic!("Default length password generation should succeed.");
  }
}

#[test]
fn test_password_custom_length() {
  if let Ok(output) = run_app(&["-l", "12"]) {
    assert_eq!(output.trim().len(), 12);
  } else {
    panic!("Custom length password generation with '-l' flag should succeed.");
  }

  if let Ok(output) = run_app(&["--length=12"]) {
    assert_eq!(output.trim().len(), 12);
  } else {
    panic!(
      "Custom length password generation with '--length' flag should succeed."
    );
  }
}

#[test]
fn test_reject_short_password() {
  const ERR_MESSAGE: &str = "Short password length should be rejected.";

  if let Err(err) = run_app(&["-l", "6"]) {
    assert!(err.contains("Password length must be at least 8 characters."));
  } else {
    panic!("{}", ERR_MESSAGE);
  }

  if let Err(err) = run_app(&["--length=6"]) {
    assert!(err.contains("Password length must be at least 8 characters."));
  } else {
    panic!("{}", ERR_MESSAGE);
  }
}

fn count_chars<F>(input: &str, f: F) -> usize
where
  F: Fn(&char) -> bool,
{
  input.chars().filter(f).count()
}

#[test]
fn test_minimum_character_types() {
  if let Ok(output) = run_app(&[
    "-l",
    "12",
    "--min-upper=2",
    "--min-lower=2",
    "--min-digit=2",
    "--min-special=2",
  ]) {
    let password = output.trim();
    assert!(count_chars(&password, |c| c.is_uppercase()) >= 2);
    assert!(count_chars(&password, |c| c.is_lowercase()) >= 2);
    assert!(count_chars(&password, |c| c.is_digit(10)) >= 2);
    assert!(count_chars(&password, |c| SPECIAL_CHARS.contains(c)) >= 2);
  } else {
    panic!("Password should contain at least 2 characters from each category.");
  }
}

#[test]
fn test_excluded_characters() {
  if let Ok(output) = run_app(&["--exclude=ABCDE12345"]) {
    let password = output.trim();
    assert!(!password.contains('A'));
    assert!(!password.contains('B'));
    assert!(!password.contains('C'));
    assert!(!password.contains('D'));
    assert!(!password.contains('E'));
    assert!(!password.contains('1'));
    assert!(!password.contains('2'));
    assert!(!password.contains('3'));
    assert!(!password.contains('4'));
    assert!(!password.contains('5'));
  } else {
    panic!(
      "Exclusion of specified characters in password generation should succeed."
    );
  }
}

fn test_exclusion_logic(exclude_chars: &str, expected_chars: &[char]) {
  if let Ok(output) = run_app(&["--exclude", exclude_chars]) {
    let password = output.trim();
    for &char in expected_chars {
      assert!(
        !password.contains(char),
        "Password should not contain the excluded character '{}'",
        char
      );
    }
  } else {
    panic!(
      "Exclusion of characters '{}' should succeed.",
      exclude_chars
    );
  }
}

#[test]
fn test_exclusion_of_single_character() {
  test_exclusion_logic("Z", &['Z']);
}

#[test]
fn test_exclusion_of_some_special_characters() {
  let exclude_chars = "#$%&";
  test_exclusion_logic(
    exclude_chars,
    &exclude_chars.chars().collect::<Vec<_>>(),
  );
}

#[test]
fn test_exclusion_of_all_special_characters() {
  let exclude_chars = SPECIAL_CHARS.iter().collect::<String>();
  test_exclusion_logic(&exclude_chars, SPECIAL_CHARS);
}

#[test]
fn test_help_option() {
  if let Ok(output) = run_app(&["--help"]) {
    assert!(output.contains("Usage: pwdg [OPTIONS]"));
  } else {
    panic!("Displaying help information should succeed.");
  }
}

#[test]
fn test_version_option() {
  if let Ok(output) = run_app(&["--version"]) {
    assert!(output.contains(concat!("pwdg ", env!("CARGO_PKG_VERSION"))))
  } else {
    panic!("Displaying version information should succeed.");
  }
}

#[test]
fn test_invalid_argument() {
  if let Err(err) = run_app(&["--invalid"]) {
    assert!(err.contains("error: unexpected argument"));
  } else {
    panic!("Invalid argument handling should produce an error.");
  }
}

#[test]
fn test_strong_password_option() {
  if let Ok(output) = run_app(&["--strong"]) {
    let password = output.trim();

    assert!(
      password.chars().any(|c| c.is_uppercase()),
      "Password must contain at least one uppercase letter."
    );
    assert!(
      password.chars().any(|c| c.is_lowercase()),
      "Password must contain at least one lowercase letter."
    );
    assert!(
      password.chars().any(|c| c.is_digit(10)),
      "Password must contain at least one digit."
    );
    assert!(
      password.chars().any(|c| SPECIAL_CHARS.contains(&c)),
      "Password must contain at least one special character."
    );
  } else {
    panic!("Strong password generation should succeed.");
  }
}

#[test]
fn test_combined_options_length_and_exclusion() {
  let exclude_chars = "ABCD";
  let length = "15";

  if let Ok(output) = run_app(&["-l", length, "--exclude", exclude_chars]) {
    let password = output.trim();
    assert_eq!(password.len(), 15, "Password length should be 15.");
    for char in exclude_chars.chars() {
      assert!(
        !password.contains(char),
        "Password should not contain the excluded character '{}'",
        char
      );
    }
  } else {
    panic!(concat!(
      "Password generation with combined length and exclusion options should",
      " succeed."
    ));
  }
}
