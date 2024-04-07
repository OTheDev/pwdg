/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0
*/
use rand::{rngs::OsRng, seq::SliceRandom};
use std::collections::HashSet;

use crate::util::checked_sum;
use crate::util::filtered_range;
use crate::Error;
use crate::SPECIAL_CHARS;

pub const MIN_LENGTH: usize = 8;
pub const DEFAULT_PWDGEN_OPTIONS: PwdGenOptions = PwdGenOptions::default_();

/// Configuration options for a password generator.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PwdGenOptions<'a> {
  pub min_upper: usize,
  pub min_lower: usize,
  pub min_digit: usize,
  pub min_special: usize,
  pub exclude: Option<&'a str>,
}

impl<'a> PwdGenOptions<'a> {
  const fn default_() -> Self {
    PwdGenOptions {
      min_upper: 0,
      min_lower: 0,
      min_digit: 0,
      min_special: 0,
      exclude: None,
    }
  }
}

impl<'a> Default for PwdGenOptions<'a> {
  /// Default constructor for `PwdGenOptions`.
  ///
  /// Sets the minimum count of each character type to `0` and does not exclude
  /// any characters.
  fn default() -> Self {
    PwdGenOptions::default_()
  }
}

struct CharacterSet {
  upper: Vec<char>,
  lower: Vec<char>,
  digit: Vec<char>,
  special: Vec<char>,
}

/// Password generator struct.
pub struct PwdGen<'a> {
  length: usize,
  options: PwdGenOptions<'a>,
  // TODO?: charset same as union of upper, lower, digit, special.
  charset: Vec<char>,
  upper: Vec<char>,
  lower: Vec<char>,
  digit: Vec<char>,
  special: Vec<char>,
}

impl<'a> PwdGen<'a> {
  /// Creates a new password generator.
  ///
  /// # Parameters
  ///
  /// - `length`: The desired length of generated passwords. Must be at least
  ///   `MIN_LENGTH`. Default is `MIN_LENGTH`.
  /// - `options`: Optional `PwdGenOptions` specifying constraints for password
  ///   generation, such as minimum numbers of different character types and
  ///   characters to exclude. If `None` is provided, default options are used.
  ///
  /// # Returns
  ///
  /// Returns a `Result<PwdGen, Error>`, where `PwdGen` is the initialized
  /// password generator if no errors are encountered.
  pub fn new(
    length: usize,
    options: Option<PwdGenOptions<'a>>,
  ) -> Result<Self, Error> {
    let options = options.unwrap_or_default();

    let cset = Self::validate_input(length, &options)?;

    let charset = [
      &cset.upper[..],
      &cset.lower[..],
      &cset.digit[..],
      &cset.special[..],
    ]
    .concat();

    Ok(PwdGen {
      length,
      options,
      charset,
      upper: cset.upper,
      lower: cset.lower,
      digit: cset.digit,
      special: cset.special,
    })
  }

  /// Generates a random password, respecting the constraints specified in the
  /// constructor.
  pub fn gen(&self) -> String {
    let mut chars: Vec<char> = Vec::with_capacity(self.length);

    Self::add_random_chars(&mut chars, &self.upper, self.options.min_upper);
    Self::add_random_chars(&mut chars, &self.lower, self.options.min_lower);
    Self::add_random_chars(&mut chars, &self.digit, self.options.min_digit);
    Self::add_random_chars(&mut chars, &self.special, self.options.min_special);

    chars.extend(
      std::iter::repeat_with(|| {
        *self
          .charset
          .choose(&mut OsRng)
          .expect("Filtered charset is nonempty")
      })
      .take(self.length - chars.len()),
    );

    chars.shuffle(&mut OsRng);

    chars.into_iter().collect()
  }

  fn add_random_chars(chars: &mut Vec<char>, range: &[char], count: usize) {
    chars.extend((0..count).filter_map(|_| range.choose(&mut OsRng)));
  }

  fn validate_input(
    length: usize,
    options: &PwdGenOptions,
  ) -> Result<CharacterSet, Error> {
    if length < MIN_LENGTH {
      return Err(Error::Length);
    }

    let min_total = checked_sum(
      [
        options.min_upper,
        options.min_lower,
        options.min_digit,
        options.min_special,
      ]
      .iter()
      .cloned(),
    );
    if min_total.is_none() || min_total.unwrap() > length {
      return Err(Error::MinLimitExceeded);
    }

    let exclude: Option<HashSet<char>> =
      Some(options.exclude.unwrap_or("").chars().collect());

    let upper = filtered_range('A'..='Z', &exclude);
    if upper.len() < options.min_upper {
      return Err(Error::InsufficientCharacters("upper"));
    }
    let lower = filtered_range('a'..='z', &exclude);
    if lower.len() < options.min_lower {
      return Err(Error::InsufficientCharacters("lower"));
    }
    let digit = filtered_range('0'..='9', &exclude);
    if digit.len() < options.min_digit {
      return Err(Error::InsufficientCharacters("digit"));
    }
    let special = filtered_range(SPECIAL_CHARS.iter().cloned(), &exclude);
    if special.len() < options.min_special {
      return Err(Error::InsufficientCharacters("special"));
    }

    Ok(CharacterSet {
      upper,
      lower,
      digit,
      special,
    })
  }

  pub fn length(&self) -> usize {
    self.length
  }

  pub fn options(&self) -> &PwdGenOptions {
    &self.options
  }
}

pub fn gen(
  length: usize,
  options: Option<PwdGenOptions>,
) -> Result<String, Error> {
  let pwdgen = PwdGen::new(length, options)?;
  Ok(pwdgen.gen())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_password_length() {
    let length = 10;
    let pwdgen = PwdGen::new(length, None).unwrap();
    let password = pwdgen.gen();
    assert_eq!(password.len(), length);
  }

  #[test]
  fn test_minimum_length_password() {
    let pwdgen = PwdGen::new(MIN_LENGTH, None).unwrap();
    let password = pwdgen.gen();
    assert_eq!(password.len(), MIN_LENGTH);
  }

  #[test]
  fn test_long_password() {
    let length = 100;
    let pwdgen = PwdGen::new(length, None).unwrap();
    let password = pwdgen.gen();
    assert_eq!(password.len(), length);
  }

  #[test]
  fn test_error_on_short_password() {
    let pwdgen = PwdGen::new(7, None);
    assert!(matches!(pwdgen, Err(Error::Length)));
  }

  #[test]
  fn test_sum_of_minimums_exceeds_length_error() {
    let options = PwdGenOptions {
      min_upper: 3,
      min_lower: 3,
      min_digit: 3,
      min_special: 3,
      exclude: None,
    };
    let pwdgen = PwdGen::new(10, Some(options));
    assert!(matches!(pwdgen, Err(Error::MinLimitExceeded)));
  }

  #[test]
  fn test_password_with_custom_options() {
    let options = PwdGenOptions {
      min_upper: 3,
      min_lower: 3,
      min_digit: 3,
      min_special: 3,
      exclude: None,
    };

    let pwdgen = PwdGen::new(15, Some(options)).unwrap();
    let password = pwdgen.gen();

    assert!(password.chars().filter(|c| c.is_uppercase()).count() >= 3);
    assert!(password.chars().filter(|c| c.is_lowercase()).count() >= 3);
    assert!(password.chars().filter(|c| c.is_digit(10)).count() >= 3);
    assert!(
      password
        .chars()
        .filter(|c| SPECIAL_CHARS.contains(&c))
        .count()
        >= 3
    );
  }

  #[test]
  fn test_password_excluding_characters() {
    let exclude = "Aa1@";
    let options = PwdGenOptions {
      min_upper: 2,
      min_lower: 2,
      min_digit: 2,
      min_special: 2,
      exclude: Some(exclude),
    };

    let pwdgen = PwdGen::new(12, Some(options)).unwrap();
    let password = pwdgen.gen();

    assert!(!password.contains('A'));
    assert!(!password.contains('a'));
    assert!(!password.contains('1'));
    assert!(!password.contains('@'));
  }

  #[test]
  fn test_exact_sum_of_minimums_equals_length() {
    let length = 12;
    let min_count = 3;
    let options = PwdGenOptions {
      min_upper: min_count,
      min_lower: min_count,
      min_digit: min_count,
      min_special: min_count,
      exclude: None,
    };

    let pwdgen = PwdGen::new(length, Some(options)).unwrap();
    let password = pwdgen.gen();

    assert_eq!(password.len(), 12);

    assert_eq!(
      password.chars().filter(|c| c.is_uppercase()).count(),
      min_count
    );
    assert_eq!(
      password.chars().filter(|c| c.is_lowercase()).count(),
      min_count
    );
    assert_eq!(
      password.chars().filter(|c| c.is_digit(10)).count(),
      min_count
    );
    assert_eq!(
      password
        .chars()
        .filter(|c| SPECIAL_CHARS.contains(&c))
        .count(),
      min_count
    );
  }

  #[test]
  fn validate_input_short_length() {
    let options = PwdGenOptions::default();
    assert!(matches!(
      PwdGen::validate_input(7, &options),
      Err(Error::Length)
    ));
  }

  #[test]
  fn validate_input_min_sum_exceeds_length() {
    let options = PwdGenOptions {
      min_upper: 3,
      min_lower: 3,
      min_digit: 3,
      min_special: 3,
      ..Default::default()
    };
    assert!(matches!(
      PwdGen::validate_input(10, &options),
      Err(Error::MinLimitExceeded)
    ));
  }

  #[test]
  fn validate_input_insufficient_upper_chars() {
    let exclude: String = ('A'..='Z').collect();
    let options = PwdGenOptions {
      min_upper: 1,
      exclude: Some(&exclude),
      ..Default::default()
    };
    assert!(matches!(
      PwdGen::validate_input(10, &options),
      Err(Error::InsufficientCharacters("upper"))
    ));
  }

  #[test]
  fn validate_input_insufficient_lower_chars() {
    let exclude: String = ('a'..='z').collect();
    let options = PwdGenOptions {
      min_lower: 1,
      exclude: Some(&exclude),
      ..Default::default()
    };
    assert!(matches!(
      PwdGen::validate_input(10, &options),
      Err(Error::InsufficientCharacters("lower"))
    ));
  }

  #[test]
  fn validate_input_insufficient_digit_chars() {
    let exclude: String = ('0'..='9').collect();
    let options = PwdGenOptions {
      min_digit: 1,
      exclude: Some(&exclude),
      ..Default::default()
    };
    assert!(matches!(
      PwdGen::validate_input(10, &options),
      Err(Error::InsufficientCharacters("digit"))
    ));
  }

  #[test]
  fn validate_input_insufficient_special_chars() {
    let exclude_special: String = SPECIAL_CHARS.iter().collect();
    let options = PwdGenOptions {
      min_special: 1,
      exclude: Some(&exclude_special),
      ..Default::default()
    };
    assert!(matches!(
      PwdGen::validate_input(10, &options),
      Err(Error::InsufficientCharacters("special"))
    ));
  }

  #[test]
  fn test_get_length() {
    let length = 23;
    let pwdgen = PwdGen::new(length, None).unwrap();
    assert_eq!(length, pwdgen.length());
  }

  #[test]
  fn test_get_options() {
    let exclude: String = SPECIAL_CHARS.iter().collect();
    let length = 23;
    let options = PwdGenOptions {
      min_upper: 1,
      min_lower: 2,
      min_digit: 3,
      min_special: 0,
      exclude: Some(&exclude),
    };
    let options_clone = options.clone();
    let pwdgen = PwdGen::new(length, Some(options)).unwrap();

    assert_eq!(options_clone, *pwdgen.options());
  }
}
