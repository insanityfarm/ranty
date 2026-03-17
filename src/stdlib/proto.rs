use super::*;

pub fn proto(vm: &mut VM, map: RantyMapHandle) -> RantyStdResult {
    vm.cur_frame_mut().write(
        map.borrow()
            .proto()
            .map_or(RantyValue::Nothing, RantyValue::Map),
    );
    Ok(())
}

pub fn set_proto(
    vm: &mut VM,
    (map, proto): (RantyMapHandle, Option<RantyMapHandle>),
) -> RantyStdResult {
    if let Some(proto) = proto.as_ref() {
        if map.would_create_proto_cycle(proto) {
            runtime_error!(
                RuntimeErrorType::ArgumentError,
                "set-proto: prototype assignment would create a cycle"
            );
        }
    }

    map.borrow_mut().set_proto(proto);
    Ok(())
}
