use super::*;

pub fn assert(vm: &mut VM, (condition, message): (bool, Option<String>)) -> RantyStdResult {
    if !condition {
        runtime_error!(
            RuntimeErrorType::AssertError,
            "{}",
            message
                .as_deref()
                .unwrap_or("assertion failed: condition was false")
        );
    }
    Ok(())
}

pub fn assert_not(vm: &mut VM, (condition, message): (bool, Option<String>)) -> RantyStdResult {
    if condition {
        runtime_error!(
            RuntimeErrorType::AssertError,
            "{}",
            message
                .as_deref()
                .unwrap_or("assertion failed: condition was true")
        );
    }
    Ok(())
}

pub fn assert_eq(
    vm: &mut VM,
    (actual, expected, message): (RantyValue, RantyValue, Option<String>),
) -> RantyStdResult {
    if expected != actual {
        runtime_error!(
            RuntimeErrorType::AssertError,
            "{}",
            message.unwrap_or_else(|| format!("expected: {}; actual: {}", expected, actual))
        );
    }
    Ok(())
}

pub fn assert_neq(
    vm: &mut VM,
    (actual, unexpected, message): (RantyValue, RantyValue, Option<String>),
) -> RantyStdResult {
    if unexpected == actual {
        runtime_error!(
            RuntimeErrorType::AssertError,
            "{}",
            message.unwrap_or_else(|| format!("unexpected value: {}", unexpected))
        );
    }
    Ok(())
}
