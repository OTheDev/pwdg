/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0
*/
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;

pub fn filtered_range<T>(
  range: impl Iterator<Item = T>,
  exclude: &Option<HashSet<T>>,
) -> Vec<T>
where
  T: Eq + Hash,
{
  match exclude {
    Some(exclusions) => range.filter(|c| !exclusions.contains(c)).collect(),
    None => range.collect(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_filtered_range_no_exclusions() {
    let range = 'a'..='c';
    let result: Vec<char> = filtered_range(range, &None);
    assert_eq!(result, vec!['a', 'b', 'c']);
  }

  #[test]
  fn test_filtered_range_with_exclusions() {
    let range = 'a'..='c';
    let exclusions: HashSet<char> = ['b'].iter().cloned().collect();
    let result: Vec<char> = filtered_range(range, &Some(exclusions));
    assert_eq!(result, vec!['a', 'c']);
  }

  #[test]
  fn test_filtered_range_empty_range() {
    let range = 'a'..'a';
    let result: Vec<char> = filtered_range(range, &None);
    assert!(result.is_empty());
  }

  #[test]
  fn test_filtered_range_full_exclusions() {
    let range = 'a'..='c';
    let exclusions: HashSet<char> = ['a', 'b', 'c'].iter().cloned().collect();
    let result: Vec<char> = filtered_range(range, &Some(exclusions));
    assert!(result.is_empty());
  }

  #[test]
  fn test_filtered_range_non_overlapping_exclusions() {
    let range = 'a'..='c';
    let exclusions: HashSet<char> = ['x', 'y', 'z'].iter().cloned().collect();
    let result: Vec<char> = filtered_range(range, &Some(exclusions));
    assert_eq!(result, vec!['a', 'b', 'c']);
  }
}
