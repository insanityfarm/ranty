use super::*;
use crate::runtime::resolver::{AttributeFrame, Reps};

pub(crate) fn get_rep_attr_value(reps: Reps) -> RantyValue {
    match reps {
        Reps::Once => RantyValue::String("once".into()),
        Reps::All => RantyValue::String("all".into()),
        Reps::Forever => RantyValue::String("forever".into()),
        Reps::Repeat(n) => RantyValue::Int(n as i64),
    }
}

pub(crate) fn set_rep_attr(attrs: &mut AttributeFrame, reps: RantyValue) -> RantyStdResult {
    attrs.reps = match reps {
        RantyValue::Int(n) => Reps::Repeat(n.max(0) as usize),
        RantyValue::String(s) => match s.as_str() {
            "once" => Reps::Once,
            "all" => Reps::All,
            "forever" => Reps::Forever,
            _ => {
                return Err(RuntimeError {
                    error_type: RuntimeErrorType::ArgumentError,
                    description: Some(format!("unknown repetition mode: '{}'", s)),
                    stack_trace: None,
                })
            }
        },
        _ => {
            return Err(RuntimeError {
                error_type: RuntimeErrorType::ArgumentError,
                description: Some(format!(
                    "value of type '{}' cannot be used as repetition value",
                    reps.type_name()
                )),
                stack_trace: None,
            })
        }
    };
    Ok(())
}

pub(crate) fn get_mutator_attr_value(mutator: Option<&RantyFunctionHandle>) -> RantyValue {
    mutator
        .cloned()
        .map(RantyValue::Function)
        .unwrap_or(RantyValue::Nothing)
}

pub(crate) fn set_mutator_attr(
    attrs: &mut AttributeFrame,
    mutator_func: RantyValue,
) -> RantyStdResult {
    attrs.mutator = match mutator_func {
        RantyValue::Function(func) => Some(func),
        RantyValue::Nothing => None,
        other => {
            return Err(RuntimeError {
                error_type: RuntimeErrorType::ValueError(ValueError::InvalidConversion {
                    from: other.type_name(),
                    to: "function",
                    message: Some("mutator must be a function or nothing".to_owned()),
                }),
                description: Some("value is not a mutator function".to_owned()),
                stack_trace: None,
            })
        }
    };
    Ok(())
}

pub(crate) fn get_selector_attr_value(selector: Option<&RantySelectorHandle>) -> RantyValue {
    selector
        .cloned()
        .map(RantyValue::Selector)
        .unwrap_or(RantyValue::Nothing)
}

pub(crate) fn set_selector_attr(
    attrs: &mut AttributeFrame,
    selector: RantyValue,
) -> RantyStdResult {
    attrs.selector = match selector {
        RantyValue::Selector(handle) => Some(handle),
        val @ RantyValue::String(_) => {
            let mode = SelectorMode::try_from_ranty(val).into_runtime_result()?;
            Some(RantySelector::new(mode).into_handle())
        }
        RantyValue::Nothing => None,
        val => {
            return Err(RuntimeError {
                error_type: RuntimeErrorType::ValueError(ValueError::InvalidConversion {
                    from: val.type_name(),
                    to: "selector",
                    message: None,
                }),
                description: Some("value is not a selector".to_owned()),
                stack_trace: None,
            })
        }
    };
    Ok(())
}

pub(crate) fn set_match_selector_attr(
    attrs: &mut AttributeFrame,
    value: RantyValue,
) -> RantyStdResult {
    attrs.selector = Some(
        RantySelector::new(SelectorMode::Match)
            .with_match_value(value)
            .into_handle(),
    );
    Ok(())
}

pub fn if_(vm: &mut VM, condition: bool) -> RantyStdResult {
    vm.resolver_mut().attrs_mut().make_if(condition);
    Ok(())
}

pub fn elseif(vm: &mut VM, condition: bool) -> RantyStdResult {
    vm.resolver_mut().attrs_mut().make_else_if(condition);
    Ok(())
}

pub fn else_(vm: &mut VM, _: ()) -> RantyStdResult {
    vm.resolver_mut().attrs_mut().make_else();
    Ok(())
}

pub fn rep(vm: &mut VM, reps: RantyValue) -> RantyStdResult {
    set_rep_attr(vm.resolver_mut().attrs_mut(), reps)
}

pub fn sep(vm: &mut VM, separator: RantyValue) -> RantyStdResult {
    vm.resolver_mut().attrs_mut().separator = separator;
    Ok(())
}

pub fn mut_(vm: &mut VM, mutator_func: Option<RantyFunctionHandle>) -> RantyStdResult {
    set_mutator_attr(
        vm.resolver_mut().attrs_mut(),
        mutator_func
            .map(RantyValue::Function)
            .unwrap_or(RantyValue::Nothing),
    )
}

pub fn step_index(vm: &mut VM, _: ()) -> RantyStdResult {
    let n = vm
        .resolver()
        .active_block()
        .map_or(0, |block| block.step_index());
    vm.cur_frame_mut().write(n as i64);
    Ok(())
}

pub fn step(vm: &mut VM, _: ()) -> RantyStdResult {
    let n = vm.resolver().active_block().map_or(0, |block| block.step());
    vm.cur_frame_mut().write(n as i64);
    Ok(())
}

pub fn step_count(vm: &mut VM, _: ()) -> RantyStdResult {
    let n = vm
        .resolver()
        .active_block()
        .map_or(0, |block| block.step_count());
    vm.cur_frame_mut().write(n as i64);
    Ok(())
}

pub fn mksel(
    vm: &mut VM,
    (mode, match_value): (SelectorMode, Option<RantyValue>),
) -> RantyStdResult {
    let sel = match mode {
        SelectorMode::Match => {
            let match_value = match_value.ok_or_else(|| RuntimeError {
                error_type: RuntimeErrorType::ArgumentError,
                description: Some("match selectors require a match value".to_owned()),
                stack_trace: None,
            })?;
            RantySelector::new(mode).with_match_value(match_value)
        }
        _ => {
            if match_value.is_some() {
                return Err(RuntimeError {
                    error_type: RuntimeErrorType::ArgumentError,
                    description: Some(format!(
                        "selector mode '{}' does not accept a match value",
                        format!("{mode:?}").to_ascii_lowercase()
                    )),
                    stack_trace: None,
                });
            }
            RantySelector::new(mode)
        }
    };
    vm.cur_frame_mut().write(sel);
    Ok(())
}

pub fn sel(vm: &mut VM, selector: Option<RantyValue>) -> RantyStdResult {
    match selector {
        Some(selector) => {
            set_selector_attr(vm.resolver_mut().attrs_mut(), selector)?;
        }
        None => {
            let selector = get_selector_attr_value(vm.resolver().attrs().selector.as_ref());
            vm.cur_frame_mut().write(selector);
        }
    }
    Ok(())
}

pub fn sel_skip(
    vm: &mut VM,
    (selector, n): (RantySelectorHandle, Option<usize>),
) -> RantyStdResult {
    let mut sel = selector.borrow_mut();
    if sel.mode() == SelectorMode::Match {
        return Err(RuntimeError {
            error_type: RuntimeErrorType::SelectorError(SelectorError::UnsupportedOperation(
                "match selectors do not support sel-skip",
            )),
            description: None,
            stack_trace: None,
        });
    }
    let count = sel.count();
    let n = n.unwrap_or(1);
    for _ in 0..n {
        sel.select(count, vm.rng()).into_runtime_result()?;
    }
    Ok(())
}

pub fn sel_freeze(
    vm: &mut VM,
    (selector, frozen): (RantySelectorHandle, Option<bool>),
) -> RantyStdResult {
    let mut sel = selector.borrow_mut();
    if sel.mode() == SelectorMode::Match {
        return Err(RuntimeError {
            error_type: RuntimeErrorType::SelectorError(SelectorError::UnsupportedOperation(
                "match selectors do not support sel-freeze",
            )),
            description: None,
            stack_trace: None,
        });
    }
    sel.set_frozen(frozen.unwrap_or(true));
    Ok(())
}

pub fn sel_frozen(vm: &mut VM, (selector,): (RantySelectorHandle,)) -> RantyStdResult {
    let sel = selector.borrow();
    if sel.mode() == SelectorMode::Match {
        return Err(RuntimeError {
            error_type: RuntimeErrorType::SelectorError(SelectorError::UnsupportedOperation(
                "match selectors do not support sel-frozen",
            )),
            description: None,
            stack_trace: None,
        });
    }
    vm.cur_frame_mut().write(sel.is_frozen());
    Ok(())
}

pub fn match_(vm: &mut VM, (value,): (RantyValue,)) -> RantyStdResult {
    set_match_selector_attr(vm.resolver_mut().attrs_mut(), value)
}

pub fn reset_attrs(vm: &mut VM, _: ()) -> RantyStdResult {
    vm.resolver_mut().reset_attrs();
    Ok(())
}
