use crate::Result;

pub fn arity(expected_arity: usize, actual_arity: usize) -> Result<()> {
    if expected_arity == actual_arity {
        Ok(())
    } else {
        Err(format!(
            "expected {expected_arity} arguments, got {actual_arity}",
        ))
    }
}
