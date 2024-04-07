/*
Copyright 2024 Owain Davies
SPDX-License-Identifier: Apache-2.0
*/
pub trait CheckedAdd: Sized {
  fn checked_add(&self, other: Self) -> Option<Self>;
}

pub fn checked_sum<I, T>(iter: I) -> Option<T>
where
  I: Iterator<Item = T>,
  T: CheckedAdd + Default,
{
  let mut acc = T::default();
  for item in iter {
    acc = match acc.checked_add(item) {
      Some(sum) => sum,
      None => return None,
    };
  }
  Some(acc)
}

macro_rules! impl_checked_add {
  ($($t:ty),*) => {
    $(impl CheckedAdd for $t {
      fn checked_add(&self, other: Self) -> Option<Self> {
        <$t>::checked_add(*self, other)
      }
    })*
  }
}

impl_checked_add!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_checked_sum_success() {
    assert_eq!(checked_sum([1 as u8, 2, 3].iter().cloned()), Some(6));
    assert_eq!(
      checked_sum([100 as u16, 200, 300].iter().cloned()),
      Some(600)
    );
    assert_eq!(
      checked_sum([u32::MAX - 200, 100, 100].iter().cloned()),
      Some(u32::MAX)
    );
  }

  #[test]
  fn test_checked_sum_overflow() {
    assert_eq!(checked_sum([usize::MAX, 1].iter().cloned()), None);
    assert_eq!(checked_sum([u32::MAX, 1].iter().cloned()), None);
    assert_eq!(checked_sum([u32::MAX, u32::MAX, 578].iter().cloned()), None);
    assert_eq!(
      checked_sum([u32::MAX - 60000, 30000, 30001].iter().cloned()),
      None
    );
  }
}
