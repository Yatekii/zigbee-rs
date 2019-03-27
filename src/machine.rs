use crate::machine_state::MachineState;
use crate::state::CommissioningMode;
use crate::init::{
    load_data,
    attempt_rejoin,
    broadcast_device_announce,
    try_touchlink,
    try_network_steering,
    try_network_formation,
    try_finding_and_binding,
};

pub fn process(state: MachineState) -> MachineState {
    match state {
        MachineState::RestorePersistentData => load_data(),
        MachineState::AttemptRejoin(state) => attempt_rejoin(state),
        MachineState::BroadcastDeviceAnnounce(state) => broadcast_device_announce(state),
        MachineState::CommissioningBegin(state) => begin_commissioning(state),
        MachineState::Commissioning(state, CommissioningMode::Touchlink) => try_touchlink(state),
        MachineState::Commissioning(state, CommissioningMode::NetworkSteering) => try_network_steering(state),
        MachineState::Commissioning(state, CommissioningMode::NetworkFormation) => try_network_formation(state),
        MachineState::Commissioning(state, CommissioningMode::FindingAndBinding) => try_finding_and_binding(state),
    }
}