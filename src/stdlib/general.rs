use data::DataSourceError;

use super::*;
use crate::lang::VarAccessMode;

/// Prints the first argument that isn't of type `nothing`.
pub fn alt(vm: &mut VM, (a, mut b): (RantyValue, RequiredVarArgs<RantyValue>)) -> RantyStdResult {
    if !a.is_nothing() {
        vm.cur_frame_mut().write(a);
        Ok(())
    } else {
        for val in b.drain(..) {
            if !val.is_nothing() {
                vm.cur_frame_mut().write(val);
                break;
            }
        }
        Ok(())
    }
}

/// Calls a function with the specified arguments.
pub fn call(
    vm: &mut VM,
    (func, args): (RantyFunctionHandle, Option<Vec<RantyValue>>),
) -> RantyStdResult {
    vm.push_val(RantyValue::Function(func.clone()))?;
    let argc = args.as_ref().map(|args| args.len()).unwrap_or(0);
    if let Some(mut args) = args {
        for arg in args.drain(..).rev() {
            vm.push_val(arg)?;
        }
    }
    vm.cur_frame_mut().push_intent(Intent::Call {
        argc,
        override_print: false,
    });
    Ok(())
}

/// Combines and prints the specified values.
pub fn cat(vm: &mut VM, mut args: VarArgs<RantyValue>) -> RantyStdResult {
    let frame = vm.cur_frame_mut();
    for val in args.drain(..) {
        frame.write(val);
    }

    Ok(())
}

/// Prints the specified values to the calling scope.
pub fn print(vm: &mut VM, mut args: VarArgs<RantyValue>) -> RantyStdResult {
    if args.len() < 2 {
        let frame = vm.cur_frame_mut();
        for val in args.drain(..) {
            frame.write(val);
        }
    } else if let Some(frame) = vm.parent_frame_mut(1) {
        for val in args.drain(..) {
            frame.write(val);
        }
    }

    Ok(())
}

/// Returns a copy of a value.
pub fn copy(vm: &mut VM, val: RantyValue) -> RantyStdResult {
    vm.cur_frame_mut().write(val.shallow_copy());
    Ok(())
}

/// Prints `a` if `cond` is true, or `b` otherwise.
pub fn either(vm: &mut VM, (cond, a, b): (bool, RantyValue, RantyValue)) -> RantyStdResult {
    let val = if cond { a } else { b };
    vm.cur_frame_mut().write(val);
    Ok(())
}

/// Forks the RNG with the specified seed.
pub fn fork(vm: &mut VM, seed: Option<RantyValue>) -> RantyStdResult {
    let rng = match seed {
        Some(RantyValue::Int(i)) => vm.rng().fork_i64(i),
        Some(RantyValue::String(s)) => vm.rng().fork_str(s.as_str()),
        Some(other) => runtime_error!(
            RuntimeErrorType::ArgumentError,
            "seeding fork with '{}' value is not supported",
            other.type_name()
        ),
        None => vm.rng().fork_random(),
    };
    vm.push_rng(Rc::new(rng));
    Ok(())
}

/// Prints the type name of `val`.
pub fn type_(vm: &mut VM, val: RantyValue) -> RantyStdResult {
    vm.cur_frame_mut().write_frag(val.type_name());
    Ok(())
}

/// Unforks the RNG down one level.
pub fn unfork(vm: &mut VM, _: ()) -> RantyStdResult {
    if vm.pop_rng().is_none() {
        runtime_error!(
            RuntimeErrorType::InvalidOperation,
            "cannot unfork root seed"
        );
    }
    Ok(())
}

/// Does nothing and takes any number of arguments. Use this as a no-op or non-printing temporal pipe.
pub fn tap(vm: &mut VM, _: VarArgs<RantyNothing>) -> RantyStdResult {
    Ok(())
}

/// Prints the RNG seed currently in use.
pub fn seed(vm: &mut VM, _: ()) -> RantyStdResult {
    let signed_seed = i64::from_ne_bytes(vm.rng().seed().to_ne_bytes());
    let frame = vm.cur_frame_mut();
    frame.write(RantyValue::Int(signed_seed));
    Ok(())
}

/// Prints the length of `val`.
pub fn len(vm: &mut VM, val: RantyValue) -> RantyStdResult {
    vm.cur_frame_mut()
        .write(val.len().try_into_ranty().into_runtime_result()?);
    Ok(())
}

/// Raises a runtime error.
pub fn error(vm: &mut VM, msg: Option<String>) -> RantyStdResult {
    const DEFAULT_ERROR_MESSAGE: &str = "user error";
    Err(RuntimeError {
        error_type: RuntimeErrorType::UserError,
        description: msg,
        stack_trace: None,
    })
}

/// Generates a `range` value with an inclusive start bound and exclusive end bound.
pub fn range(vm: &mut VM, (a, b, step): (i64, Option<i64>, Option<u64>)) -> RantyStdResult {
    let step = step.unwrap_or(1);

    let range = if let Some(b) = b {
        RantyRange::new(a, b, step)
    } else {
        RantyRange::new(0, a, step)
    };

    vm.cur_frame_mut().write(range);
    Ok(())
}

/// Generates a `range` value with inclusive bounds.
pub fn irange(vm: &mut VM, (a, b, step): (i64, Option<i64>, Option<u64>)) -> RantyStdResult {
    let step = step.unwrap_or(1);

    let range = if let Some(b) = b {
        RantyRange::new(a, b + if a <= b { 1 } else { -1 }, step)
    } else {
        RantyRange::new(0, a + if a >= 0 { 1 } else { -1 }, step)
    };

    vm.cur_frame_mut().write(range);
    Ok(())
}

/// Imports a module.
pub fn require(vm: &mut VM, module_path: String) -> RantyStdResult {
    let dependant = Rc::clone(vm.cur_frame().origin());
    let request_key = module_request_cache_key(module_path.as_str(), Some(dependant.as_ref()));

    // Get name of module from path
    if let Some(module_name) = PathBuf::from(&module_path)
        .with_extension("")
        .file_name()
        .map(|name| name.to_str())
        .flatten()
        .map(|name| name.to_owned())
    {
        // Check if module is cached; if so, don't do anything
        if let Some(cached_module) = vm.context().get_cached_module(request_key.as_str()) {
            vm.def_var_value(
                module_name.as_str(),
                VarAccessMode::Descope(1),
                cached_module,
                true,
            )?;
            return Ok(());
        }

        if vm.loading_modules.contains(request_key.as_str()) {
            runtime_error!(
                RuntimeErrorType::ModuleError(ModuleResolveError {
                    name: module_path.clone(),
                    reason: ModuleResolveErrorReason::FileIOError(IOErrorKind::InvalidData),
                }),
                "cyclic module import detected for '{}'",
                module_path
            );
        }

        // If not cached, attempt to resolve it and load the module
        let module_resolver = Rc::clone(&vm.context().module_resolver);
        match module_resolver.try_resolve(
            vm.context_mut(),
            module_path.as_str(),
            Some(dependant.as_ref()),
        ) {
            Ok(module_program) => {
                let mut cache_keys = vec![request_key.clone()];

                if let Some(resolved_key) = module_resolved_cache_key(&module_program) {
                    if let Some(cached_module) =
                        vm.context().get_cached_module(resolved_key.as_str())
                    {
                        vm.context_mut()
                            .cache_module(request_key.as_str(), cached_module.clone());
                        vm.def_var_value(
                            module_name.as_str(),
                            VarAccessMode::Descope(1),
                            cached_module,
                            true,
                        )?;
                        return Ok(());
                    }

                    if vm.loading_modules.contains(resolved_key.as_str()) {
                        runtime_error!(
                            RuntimeErrorType::ModuleError(ModuleResolveError {
                                name: module_path.clone(),
                                reason: ModuleResolveErrorReason::FileIOError(
                                    IOErrorKind::InvalidData
                                ),
                            }),
                            "cyclic module import detected for '{}'",
                            module_path
                        );
                    }

                    if resolved_key != request_key {
                        cache_keys.push(resolved_key);
                    }
                }

                for cache_key in &cache_keys {
                    vm.loading_modules.insert(cache_key.clone());
                }

                vm.cur_frame_mut().push_intent(Intent::ImportLastAsModule {
                    module_name,
                    descope: 1,
                    cache_keys,
                });
                vm.push_frame_flavored(
                    Rc::clone(&module_program.root),
                    StackFrameFlavor::FunctionBody,
                )?;
                Ok(())
            }
            Err(err) => runtime_error!(RuntimeErrorType::ModuleError(err)),
        }
    } else {
        runtime_error!(
            RuntimeErrorType::ArgumentError,
            "missing module name from path: {}",
            module_path
        );
    }
}

/// Attempts to call the `context` function. If it raises a runtime error, calls the `handler` function and passes in the error information.
pub fn try_(
    vm: &mut VM,
    (context, handler): (RantyFunctionHandle, Option<RantyFunctionHandle>),
) -> RantyStdResult {
    vm.push_unwind_state(handler);
    vm.cur_frame_mut().push_intent(Intent::DropStaleUnwinds);
    vm.call_func(context, vec![], false)?;
    Ok(())
}

/// Requests data from a data source whose ID matches `dsid`.
pub fn ds_request(
    vm: &mut VM,
    (dsid, args): (InternalString, VarArgs<RantyValue>),
) -> RantyStdResult {
    match vm.context().data_source(dsid.as_str()) {
        Some(ds) => {
            let result = ds.request_data(args.into_vec()).into_runtime_result()?;
            vm.cur_frame_mut().write(result);
        }
        None => {
            runtime_error!(RuntimeErrorType::DataSourceError(DataSourceError::User(
                format!("data source '{}' not found", &dsid)
            )))
        }
    }
    Ok(())
}

/// Prints a list of available data source IDs.
pub fn ds_query_sources(vm: &mut VM, _: ()) -> RantyStdResult {
    let sources = vm
        .context()
        .iter_data_sources()
        .map(|(id, _)| id.into_ranty())
        .collect::<RantyList>();
    vm.cur_frame_mut().write(sources);
    Ok(())
}
