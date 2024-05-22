use anyhow::anyhow;

use crate::Result;

pub fn try_as_array<const N: usize, T>(values: &[T]) -> Result<&[T; N]> {
    let actual_len = values.len();
    values
        .try_into()
        .map_err(|_| anyhow!("expected {N} values, got {actual_len}"))
}
