use std::cmp::Ordering;

use super::*;
use crate::lang::Slice;

pub fn list(vm: &mut VM, items: VarArgs<RantyValue>) -> RantyStdResult {
    vm.cur_frame_mut()
        .write(items.iter().cloned().collect::<RantyList>());
    Ok(())
}

pub fn tuple(vm: &mut VM, items: VarArgs<RantyValue>) -> RantyStdResult {
    vm.cur_frame_mut()
        .write(items.iter().cloned().collect::<RantyTuple>());
    Ok(())
}

pub fn nlist(vm: &mut VM, items: VarArgs<RantyValue>) -> RantyStdResult {
    let list = RantyValue::List(items.iter().cloned().collect::<RantyList>().into_handle());
    vm.cur_frame_mut().write(vec![list]);
    Ok(())
}

pub fn rev(vm: &mut VM, val: RantyValue) -> RantyStdResult {
    vm.cur_frame_mut().write(val.reversed());
    Ok(())
}

pub fn squish_self(vm: &mut VM, (list, target_size): (RantyListHandle, usize)) -> RantyStdResult {
    let mut list = list.borrow_mut();

    if target_size == 0 {
        runtime_error!(
            RuntimeErrorType::ArgumentError,
            "cannot squish to a target size of 0"
        );
    }

    if list.len() <= target_size || list.len() < 2 {
        return Ok(());
    }

    let rng = vm.rng();
    while list.len() > target_size {
        let n = list.len();
        let left_index = rng.next_usize(n - 1);
        let right_index = left_index + 1;
        let left_val = list.get(left_index).unwrap().clone();
        let right_val = list.remove(right_index);
        list[left_index] = left_val + right_val;
    }

    Ok(())
}

pub fn squish(vm: &mut VM, (list, target_size): (RantyListHandle, usize)) -> RantyStdResult {
    let list = list.cloned();
    let list_ref_clone = RantyListHandle::clone(&list);
    squish_self(vm, (list, target_size))?;
    vm.cur_frame_mut().write(list_ref_clone);
    Ok(())
}

pub fn squish_thru(vm: &mut VM, (list, target_size): (RantyListHandle, usize)) -> RantyStdResult {
    let list_ref_clone = RantyListHandle::clone(&list);
    squish_self(vm, (list, target_size))?;
    vm.cur_frame_mut().write(list_ref_clone);
    Ok(())
}

pub fn filter(
    vm: &mut VM,
    (list, predicate): (RantyListHandle, RantyFunctionHandle),
) -> RantyStdResult {
    let list_ref = list.borrow();
    if list_ref.is_empty() {
        vm.cur_frame_mut().write(list_ref.clone());
        return Ok(());
    }

    fn _iterate_filter(
        vm: &mut VM,
        src: RantyListHandle,
        mut dest: RantyList,
        index: usize,
        predicate: RantyFunctionHandle,
    ) -> RuntimeResult<()> {
        let src_ref = src.borrow();

        // Check predicate result from last iteration
        if index > 0 {
            match vm.pop_val()? {
                RantyValue::Boolean(passed) => {
                    if passed {
                        dest.push(src_ref.get(index - 1).cloned().unwrap_or_default());
                    }
                }
                other => runtime_error!(
                    RuntimeErrorType::TypeError,
                    "filter callback expected to return 'bool' value, but returned '{}' instead",
                    other.type_name()
                ),
            }
        }

        // Check if filtering finished
        if index >= src_ref.len() {
            vm.cur_frame_mut().write(dest);
            return Ok(());
        }

        let src_clone = RantyListHandle::clone(&src);
        let predicate_arg = src_ref.get(index).cloned().unwrap_or_default();
        let predicate_clone = Rc::clone(&predicate);

        // Prepare next iteration
        vm.cur_frame_mut().push_intent(Intent::RuntimeCall {
            function: Box::new(move |vm| {
                _iterate_filter(vm, src_clone, dest, index + 1, predicate)?;
                Ok(())
            }),
            interrupt: false,
        });

        // Prepare predicate call for current iteration
        vm.push_val(RantyValue::Function(predicate_clone))?;
        vm.push_val(predicate_arg)?;
        vm.cur_frame_mut().push_intent(Intent::Call {
            argc: 1,
            override_print: true,
        });

        Ok(())
    }

    let list_clone = RantyListHandle::clone(&list);
    vm.cur_frame_mut().push_intent(Intent::RuntimeCall {
        function: Box::new(move |vm| {
            _iterate_filter(vm, list_clone, RantyList::new(), 0, predicate)
        }),
        interrupt: false,
    });

    Ok(())
}

pub fn map(vm: &mut VM, (list, map_func): (RantyListHandle, RantyFunctionHandle)) -> RantyStdResult {
    let is_list_empty = list.borrow().is_empty();
    if is_list_empty {
        vm.cur_frame_mut().write(list);
        return Ok(());
    }

    let list_ref = list.borrow();

    fn _iterate_map(
        vm: &mut VM,
        src: RantyListHandle,
        mut dest: RantyList,
        index: usize,
        map_func: RantyFunctionHandle,
    ) -> RuntimeResult<()> {
        let src_ref = src.borrow();

        // Add result from last iteration to destination list
        if index > 0 {
            dest.push(vm.pop_val()?);
        }

        // Check if mapping finished
        if index >= src_ref.len() {
            vm.cur_frame_mut().write(dest);
            return Ok(());
        }

        let src_clone = RantyListHandle::clone(&src);
        let map_func_arg = src_ref.get(index).cloned().unwrap_or_default();
        let map_func_clone = Rc::clone(&map_func);

        // Prepare next iteration
        vm.cur_frame_mut().push_intent(Intent::RuntimeCall {
            function: Box::new(move |vm| {
                _iterate_map(vm, src_clone, dest, index + 1, map_func)?;
                Ok(())
            }),
            interrupt: false,
        });

        // Prepare predicate call for current iteration
        vm.push_val(RantyValue::Function(map_func_clone))?;
        vm.push_val(map_func_arg)?;
        vm.cur_frame_mut().push_intent(Intent::Call {
            argc: 1,
            override_print: true,
        });

        Ok(())
    }

    let list_clone = RantyListHandle::clone(&list);
    vm.cur_frame_mut().push_intent(Intent::RuntimeCall {
        function: Box::new(move |vm| _iterate_map(vm, list_clone, RantyList::new(), 0, map_func)),
        interrupt: false,
    });

    Ok(())
}

pub fn zip(
    vm: &mut VM,
    (list_a, list_b, zip_func): (RantyListHandle, RantyListHandle, RantyFunctionHandle),
) -> RantyStdResult {
    let (list_a_ref, list_b_ref) = (list_a.borrow(), list_b.borrow());
    let max_len = list_a_ref.len().max(list_b_ref.len());

    fn _iterate_zip(
        vm: &mut VM,
        src_a: RantyListHandle,
        src_b: RantyListHandle,
        mut dest: RantyList,
        index: usize,
        max_len: usize,
        zip_func: RantyFunctionHandle,
    ) -> RuntimeResult<()> {
        let (src_a_ref, src_b_ref) = (src_a.borrow(), src_b.borrow());

        // Add result from last iteration to destination list
        if index > 0 {
            dest.push(vm.pop_val()?);
        }

        // Check whether zipping finished
        if index >= max_len {
            vm.cur_frame_mut().write(dest);
            return Ok(());
        }

        let (src_a_clone, src_b_clone) =
            (RantyListHandle::clone(&src_a), RantyListHandle::clone(&src_b));
        let zip_func_clone = Rc::clone(&zip_func);

        // Prepare next iteration
        vm.cur_frame_mut().push_intent(Intent::RuntimeCall {
            function: Box::new(move |vm| {
                _iterate_zip(
                    vm,
                    src_a_clone,
                    src_b_clone,
                    dest,
                    index + 1,
                    max_len,
                    zip_func_clone,
                )
            }),
            interrupt: false,
        });

        // Prepare predicate call for current iteration
        vm.push_val(RantyValue::Function(zip_func))?;
        vm.push_val(src_b_ref.get(index).cloned().unwrap_or_default())?;
        vm.push_val(src_a_ref.get(index).cloned().unwrap_or_default())?;
        vm.cur_frame_mut().push_intent(Intent::Call {
            argc: 2,
            override_print: true,
        });

        Ok(())
    }

    let (list_a_clone, list_b_clone) = (
        RantyListHandle::clone(&list_a),
        RantyListHandle::clone(&list_b),
    );
    vm.cur_frame_mut().push_intent(Intent::RuntimeCall {
        function: Box::new(move |vm| {
            _iterate_zip(
                vm,
                list_a_clone,
                list_b_clone,
                RantyList::new(),
                0,
                max_len,
                zip_func,
            )
        }),
        interrupt: false,
    });

    Ok(())
}

pub fn join(vm: &mut VM, (list, sep): (Vec<RantyValue>, Option<RantyValue>)) -> RantyStdResult {
    let mut is_first = true;
    let frame = vm.cur_frame_mut();
    for val in list {
        if is_first {
            is_first = false;
        } else if let Some(sep) = &sep {
            frame.write(sep.clone());
        }
        frame.write(val);
    }
    Ok(())
}

#[allow(clippy::needless_range_loop)]
pub fn oxford_join(
    vm: &mut VM,
    (comma, conj, comma_conj, list): (RantyValue, RantyValue, RantyValue, Vec<RantyValue>),
) -> RantyStdResult {
    let frame = vm.cur_frame_mut();
    let n = list.len();
    for i in 0..n {
        match (i, n, i == n - 1) {
            (0, ..) | (_, 1, _) => {}
            (1, 2, _) => frame.write(conj.clone()),
            (.., false) => frame.write(comma.clone()),
            (.., true) => frame.write(comma_conj.clone()),
        }
        frame.write(list[i].clone());
    }
    Ok(())
}

pub fn sum(vm: &mut VM, collection: RantyOrderedCollection) -> RantyStdResult {
    if collection.is_empty() {
        return Ok(());
    }

    let mut sum = RantyValue::Nothing;

    for i in 0..collection.len() {
        sum = sum + collection.index_get(i as i64).into_runtime_result()?;
    }

    vm.cur_frame_mut().write(sum);

    Ok(())
}

pub fn shuffle_self(vm: &mut VM, list: RantyListHandle) -> RantyStdResult {
    let mut list = list.borrow_mut();
    if list.is_empty() {
        return Ok(());
    }

    let n = list.len();
    let rng = vm.rng();
    for i in 0..n {
        list.swap(i, rng.next_usize(n));
    }
    Ok(())
}

pub fn shuffle_thru(vm: &mut VM, list: RantyListHandle) -> RantyStdResult {
    let list_ref_clone = RantyListHandle::clone(&list);
    shuffle_self(vm, list)?;
    vm.cur_frame_mut().write(list_ref_clone);
    Ok(())
}

pub fn shuffle(vm: &mut VM, list: RantyListHandle) -> RantyStdResult {
    let list = list.cloned();
    let list_ref_clone = RantyListHandle::clone(&list);
    shuffle_self(vm, list)?;
    vm.cur_frame_mut().write(list_ref_clone);
    Ok(())
}

pub fn clear(vm: &mut VM, collection: RantyValue) -> RantyStdResult {
    match collection {
        RantyValue::List(list) => list.borrow_mut().clear(),
        RantyValue::Map(map) => map.borrow_mut().clear(),
        _ => {
            runtime_error!(
                RuntimeErrorType::ArgumentError,
                "value passed to [clear] must be a collection type"
            );
        }
    }
    Ok(())
}

pub fn keys(vm: &mut VM, map: RantyMapHandle) -> RantyStdResult {
    vm.cur_frame_mut().write(map.borrow().raw_keys());
    Ok(())
}

pub fn values(vm: &mut VM, map: RantyMapHandle) -> RantyStdResult {
    vm.cur_frame_mut().write(map.borrow().raw_values());
    Ok(())
}

pub fn assoc(vm: &mut VM, (keys, values): (RantyListHandle, RantyListHandle)) -> RantyStdResult {
    let keys = keys.borrow();
    let values = values.borrow();
    if keys.len() != values.len() {
        runtime_error!(
            RuntimeErrorType::ArgumentError,
            "assoc: key and value counts don't match"
        );
    }

    let mut map = RantyMap::new();
    for (key, val) in keys.iter().zip(values.iter()) {
        map.raw_set(key.to_string().as_ref(), val.clone());
    }

    vm.cur_frame_mut().write(map);

    Ok(())
}

pub fn augment_self(
    vm: &mut VM,
    (to_map, from_map): (RantyMapHandle, RantyMapHandle),
) -> RantyStdResult {
    for (key, val) in from_map.borrow().raw_pairs_internal() {
        let orig_val = to_map.borrow().get(key).map(|v| v.as_ref().clone());
        if let Some(orig_val) = orig_val {
            to_map.borrow_mut().raw_set(key, orig_val + val.clone());
        } else {
            to_map.borrow_mut().raw_set(key, val.clone());
        }
    }

    Ok(())
}

pub fn augment_thru(
    vm: &mut VM,
    (to_map, from_map): (RantyMapHandle, RantyMapHandle),
) -> RantyStdResult {
    let map_ref_clone = RantyMapHandle::clone(&to_map);
    augment_self(vm, (to_map, from_map))?;
    vm.cur_frame_mut().write(map_ref_clone);
    Ok(())
}

pub fn augment(vm: &mut VM, (to_map, from_map): (RantyMapHandle, RantyMapHandle)) -> RantyStdResult {
    let to_map = to_map.cloned();
    let to_map_ref_clone = RantyMapHandle::clone(&to_map);
    augment_self(vm, (to_map_ref_clone, from_map))?;
    vm.cur_frame_mut().write(to_map);
    Ok(())
}

pub fn translate(vm: &mut VM, (list, map): (RantyListHandle, RantyMapHandle)) -> RantyStdResult {
    let list = list.borrow();
    let map = map.borrow();

    let translated: RantyList = list
        .iter()
        .map(|val| {
            map.raw_get(val.to_string().as_ref())
                .cloned()
                .unwrap_or_else(|| val.clone())
        })
        .collect();

    vm.cur_frame_mut().write(translated);

    Ok(())
}

pub fn push(vm: &mut VM, (list, value): (RantyListHandle, RantyValue)) -> RantyStdResult {
    list.borrow_mut().push(value);
    Ok(())
}

pub fn pop(vm: &mut VM, list: RantyListHandle) -> RantyStdResult {
    let value = list.borrow_mut().pop().unwrap_or(RantyValue::Nothing);
    vm.cur_frame_mut().write(value);
    Ok(())
}

pub fn sift_self(vm: &mut VM, (list, size): (RantyListHandle, usize)) -> RantyStdResult {
    let mut list = list.borrow_mut();
    if list.len() <= size {
        return Ok(());
    }

    let rng = vm.rng();
    while list.len() > size {
        let remove_index = rng.next_usize(list.len());
        list.remove(remove_index);
    }

    Ok(())
}

pub fn sift_thru(vm: &mut VM, (list, size): (RantyListHandle, usize)) -> RantyStdResult {
    let list_ref_clone = RantyListHandle::clone(&list);
    sift_self(vm, (list, size))?;
    vm.cur_frame_mut().write(list_ref_clone);
    Ok(())
}

pub fn sift(vm: &mut VM, (list, size): (RantyListHandle, usize)) -> RantyStdResult {
    let mut list = list.borrow().clone();

    let rng = vm.rng();
    while list.len() > size {
        let remove_index = rng.next_usize(list.len());
        list.remove(remove_index);
    }

    vm.cur_frame_mut().write(list);

    Ok(())
}

pub fn insert(
    vm: &mut VM,
    (collection, value, pos): (RantyValue, RantyValue, RantyValue),
) -> RantyStdResult {
    match (collection, pos) {
        // Insert into list by index
        (RantyValue::List(list), RantyValue::Int(index)) => {
            let mut list = list.borrow_mut();
            // Bounds check
            if index < 0 || index as usize > list.len() {
                runtime_error!(
                    RuntimeErrorType::IndexError(IndexError::OutOfRange),
                    "index is out of range of list size"
                );
            }
            let index = index as usize;
            list.insert(index, value);
        }
        // Error on non-index list access
        (RantyValue::List(_), non_index) => {
            runtime_error!(
                RuntimeErrorType::ArgumentError,
                "cannot insert into list by '{}' index",
                non_index.type_name()
            );
        }
        // Insert into map by key
        (RantyValue::Map(map), key_val) => {
            let mut map = map.borrow_mut();
            let key = key_val.to_string();
            map.raw_set(key.as_str(), value);
        }
        _ => {
            runtime_error!(
                RuntimeErrorType::ArgumentError,
                "cannot insert into a non-collection"
            );
        }
    }
    Ok(())
}

pub fn index_of(vm: &mut VM, (list, value): (RantyListHandle, RantyValue)) -> RantyStdResult {
    let index = list
        .borrow()
        .iter()
        .position(|v| v == &value)
        .map(|i| RantyValue::Int(i as i64))
        .unwrap_or(RantyValue::Nothing);

    vm.cur_frame_mut().write(index);
    Ok(())
}

pub fn last_index_of(vm: &mut VM, (list, value): (RantyListHandle, RantyValue)) -> RantyStdResult {
    let index = list
        .borrow()
        .iter()
        .rposition(|v| v == &value)
        .map(|i| RantyValue::Int(i as i64))
        .unwrap_or(RantyValue::Nothing);

    vm.cur_frame_mut().write(index);
    Ok(())
}

pub fn remove(vm: &mut VM, (collection, pos): (RantyValue, RantyValue)) -> RantyStdResult {
    match (collection, pos) {
        // Remove from list by index
        (RantyValue::List(list), RantyValue::Int(index)) => {
            let mut list = list.borrow_mut();
            // Bounds check
            if index < 0 || index as usize >= list.len() {
                runtime_error!(
                    RuntimeErrorType::IndexError(IndexError::OutOfRange),
                    "index is out of range of list size"
                );
            }
            let index = index as usize;
            list.remove(index);
        }
        // Error on non-index list access
        (RantyValue::List(_), non_index) => {
            runtime_error!(
                RuntimeErrorType::ArgumentError,
                "cannot remove from list by '{}' index",
                non_index.type_name()
            );
        }
        // Remove from into map by key
        (RantyValue::Map(map), key_val) => {
            let mut map = map.borrow_mut();
            let key = key_val.to_string();
            map.raw_remove(key.as_str());
        }
        _ => {
            runtime_error!(
                RuntimeErrorType::ArgumentError,
                "cannot remove from a non-collection"
            );
        }
    }
    Ok(())
}

pub fn take(vm: &mut VM, (collection, pos): (RantyValue, RantyValue)) -> RantyStdResult {
    match (collection, pos) {
        // Take from list by index
        (RantyValue::List(list), RantyValue::Int(index)) => {
            let mut list = list.borrow_mut();
            // Bounds check
            if index < 0 || index as usize >= list.len() {
                runtime_error!(
                    RuntimeErrorType::IndexError(IndexError::OutOfRange),
                    "index is out of range of list size"
                );
            }
            let index = index as usize;
            list.remove(index);
        }
        // Error on non-index list access
        (RantyValue::List(_), non_index) => {
            runtime_error!(
                RuntimeErrorType::ArgumentError,
                "cannot take from list by '{}' index",
                non_index.type_name()
            );
        }
        // Remove from into map by key
        (RantyValue::Map(map), key_val) => {
            let mut map = map.borrow_mut();
            let key = key_val.to_string();
            if let Some(val) = map.raw_take(key.as_str()) {
                vm.cur_frame_mut().write(val);
            } else {
                runtime_error!(
                    RuntimeErrorType::KeyError(KeyError::KeyNotFound(key.to_owned())),
                    "tried to take non-existent key: '{}'",
                    key
                );
            }
        }
        _ => {
            runtime_error!(
                RuntimeErrorType::ArgumentError,
                "cannot take from a non-collection"
            );
        }
    }
    Ok(())
}

pub fn sort_self(vm: &mut VM, list: RantyListHandle) -> RantyStdResult {
    let mut list = list.borrow_mut();
    list.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    Ok(())
}

pub fn sort_thru(vm: &mut VM, list: RantyListHandle) -> RantyStdResult {
    let list_ref_clone = list.clone();
    sort_self(vm, list)?;
    vm.cur_frame_mut().write(list_ref_clone);
    Ok(())
}

pub fn sort(vm: &mut VM, list: RantyListHandle) -> RantyStdResult {
    let mut list_copy = list.borrow().clone();
    list_copy.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    vm.cur_frame_mut().write(list_copy);
    Ok(())
}

pub fn has(vm: &mut VM, (value, key): (RantyValue, RantyValue)) -> RantyStdResult {
    let result = match (value, key) {
        (RantyValue::Map(map), RantyValue::String(key)) => map.borrow().raw_has_key(key.as_str()),
        (RantyValue::List(list), element) => list.borrow().contains(&element),
        (value, key) => {
            runtime_error!(
                RuntimeErrorType::ArgumentError,
                "unable to check if value of type '{}' contains element of type '{}'",
                value.type_name(),
                key.type_name()
            )
        }
    };

    vm.cur_frame_mut().write(result);
    Ok(())
}

pub fn seg(vm: &mut VM, (collection, seg_size): (RantyValue, usize)) -> RantyStdResult {
    if !collection.is_indexable() {
        runtime_error!(
            RuntimeErrorType::ArgumentError,
            "seg: type '{}' cannot be segmented",
            collection.type_name()
        )
    }

    if seg_size > 0 {
        let mut segs = vec![];
        let len = collection.len();
        let last_seg_len = len % seg_size;
        let n = len / seg_size + if last_seg_len > 0 { 1 } else { 0 };
        if last_seg_len > 0 {
            for i in 0..n {
                if i == n - 1 {
                    segs.push(
                        collection
                            .slice_get(&Slice::Between(
                                (i * seg_size) as i64,
                                (i * seg_size + last_seg_len) as i64,
                            ))
                            .into_runtime_result()?,
                    );
                } else {
                    segs.push(
                        collection
                            .slice_get(&Slice::Between(
                                (i * seg_size) as i64,
                                ((i + 1) * seg_size) as i64,
                            ))
                            .into_runtime_result()?,
                    );
                }
            }
        } else {
            for i in 0..n {
                segs.push(
                    collection
                        .slice_get(&Slice::Between(
                            (i * seg_size) as i64,
                            ((i + 1) * seg_size) as i64,
                        ))
                        .into_runtime_result()?,
                );
            }
        }
        vm.cur_frame_mut().write(segs);
    }
    Ok(())
}

pub fn chunks(vm: &mut VM, (collection, chunk_count): (RantyValue, usize)) -> RantyStdResult {
    if !collection.is_indexable() {
        runtime_error!(
            RuntimeErrorType::ArgumentError,
            "chunks: type '{}' cannot be chunked",
            collection.type_name()
        )
    }

    let mut chunks = vec![];

    if chunk_count > 0 {
        let min_chunk_size = collection.len() / chunk_count;
        let max_chunk_size = min_chunk_size + 1;
        let num_bigger_chunks = collection.len() % chunk_count;

        let collection_len = collection.len();
        for i in 0..chunk_count {
            let chunk_size = if i < num_bigger_chunks {
                max_chunk_size
            } else {
                min_chunk_size
            };
            let chunk_offset = if i < num_bigger_chunks {
                i * max_chunk_size
            } else {
                num_bigger_chunks * max_chunk_size + (i - num_bigger_chunks) * min_chunk_size
            };

            chunks.push(
                collection
                    .slice_get(&Slice::Between(
                        chunk_offset as i64,
                        (chunk_offset + chunk_size) as i64,
                    ))
                    .into_runtime_result()?,
            );
        }
    }

    vm.cur_frame_mut().write(chunks);
    Ok(())
}

pub fn fill_self(vm: &mut VM, (list, value): (RantyListHandle, RantyValue)) -> RantyStdResult {
    let len = list.borrow().len();
    let mut list = list.borrow_mut();
    for i in 0..len {
        if let Some(item) = list.get_mut(i) {
            *item = value.clone();
        }
    }
    Ok(())
}

pub fn fill_thru(vm: &mut VM, (list, value): (RantyListHandle, RantyValue)) -> RantyStdResult {
    fill_self(vm, (list.clone(), value))?;
    vm.cur_frame_mut().write(list);
    Ok(())
}
