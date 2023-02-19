use crate::{Result, Value};

#[allow(clippy::module_name_repetitions)]
pub fn try_checked<const N: usize>(values: &[Value]) -> Result<&[Value; N]> {
    let actual_len = values.len();
    values
        .try_into()
        .map_err(|_| format!("expected {N} values, got {actual_len}"))
}
