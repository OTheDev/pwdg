/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0
*/
use crate::MIN_LENGTH;

#[derive(Debug)]
pub enum Error {
  /// Specified length is less than `MIN_LENGTH`.
  Length,
  /// Sum of the minimum character requirements exceeds the length.
  MinLimitExceeded,
  /// There exists a category (upper, lower, digit, or special) such that the
  /// number of characters in that category is less than any minimum specified
  /// for that category, after applying any exclusions.
  InsufficientCharacters(&'static str),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Error::Length => {
        write!(
          f,
          "Password length must be at least {} characters. [Error::Length]",
          MIN_LENGTH
        )
      }
      Error::MinLimitExceeded => {
        write!(
          f,
          concat!(
            "Sum of minimum character requirements exceeds password length. ",
            "[Error::MinLimitExceeded]"
          )
        )
      }
      Error::InsufficientCharacters(char_type) => {
        write!(
          f,
          concat!(
            "Insufficient characters available for {}. ",
            "[Error::InsufficientCharacters]"
          ),
          char_type
        )
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_length_error_display() {
    let error = Error::Length;
    assert!(format!("{}", error).contains(&format!(
      "Password length must be at least {} characters.",
      MIN_LENGTH
    )));
  }

  #[test]
  fn test_min_limit_exceeded_error_display() {
    let error = Error::MinLimitExceeded;
    assert!(format!("{}", error).contains(
      "Sum of minimum character requirements exceeds password length."
    ));
  }

  #[test]
  fn test_insufficient_characters_error_display() {
    let error = Error::InsufficientCharacters("upper");
    assert!(format!("{}", error)
      .contains("Insufficient characters available for upper"));
  }
}
