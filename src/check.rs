use crate::{Result, Symbol, Value};

#[allow(clippy::module_name_repetitions)]
pub fn check_type(type_: Symbol, expected_type: Symbol) -> Result<()> {
    if type_ != expected_type {
        return Err(format!("expected {expected_type}, got {type_}"));
    }

    Ok(())
}

#[allow(clippy::module_name_repetitions)]
pub fn check_count(type_: Symbol, values: &[Value], expected_count: usize) -> Result<()> {
    let count = values.len();
    if count != expected_count {
        return Err(format!(
            "expected {type_} with {expected_count} values, got {count}",
        ));
    }

    Ok(())
}
