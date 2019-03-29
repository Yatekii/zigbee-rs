use crate::state::{
    State,
    DeviceType,
    CommissioningMode,
    CommissioningStatus,
    CommissioningModeFlag,
};
use crate::machine_state::MachineState;

pub fn load_data() -> MachineState {
    let state = State::new();
    match state.device_type {
        DeviceType::EndDevice => MachineState::AttemptRejoin(state),
        DeviceType::Router => if state.bdbCommissioningCapability.contains(CommissioningModeFlag::Touchlink) {
            // TODO:
            // select_channel(state);
            MachineState::InitDone(state)
        } else {
            // TODO:
            MachineState::InitDone(state)
        }
        _ => MachineState::InitDone(state),
    }
}

fn try_rejoin() -> bool {
    true
}

pub fn attempt_rejoin(state: State) -> MachineState {
    if try_rejoin() {
        MachineState::BroadcastDeviceAnnounce(state)
    } else {
        MachineState::AttemptRejoin(state)
    }
}

pub fn broadcast_device_announce(state: State) -> MachineState {
    MachineState::InitDone(state)
}

pub fn begin_commissioning(state: State) -> MachineState {
    if state.bdbCommissioningMode.contains(CommissioningModeFlag::Touchlink) {
        MachineState::Commissioning(state, CommissioningMode::Touchlink)
    } else {
        MachineState::CommissioningDone(state)
    }
}

pub fn try_touchlink(mut state: State) -> MachineState {
    // TODO: - Touchlink init
    //       - Add actual response to the state struct (bdbCommissioningStatus)
    state.bdbCommitssioningStatus = CommissioningStatus::NO_SCAN_RESPONSE;
    match state.bdbCommitssioningStatus {
        CommissioningStatus::NO_SCAN_RESPONSE => {
            if state.bdbCommissioningMode.contains(CommissioningModeFlag::NetworkSteering) {
                MachineState::Commissioning(state, CommissioningMode::NetworkSteering)
            } else {
                MachineState::CommissioningDone(state)
            }
        },
        _ => MachineState::CommissioningDone(state),
    }
}

pub fn try_network_steering(state: State) -> MachineState {
    // TODO: - NetworkSteering init
    //       - Add actual response to the state struct (bdbCommissioningStatus)
    
    if state.bdbCommissioningMode.contains(CommissioningModeFlag::NetworkFormation) {
        MachineState::Commissioning(state, CommissioningMode::NetworkFormation)
    } else {
        MachineState::CommissioningDone(state)
    }
}

fn network_steering(state: State) {
    if state.bdbNodeIsOnANetwork {
        // TODO: Perform network steering for a node on a network
    } else {
        // TODO: Perform network steering for a node not on a network
    }
}

pub fn try_network_formation(mut state: State) -> MachineState {
    // TODO: - NetworkFormation init
    //       - Add actual response to the state struct (bdbCommissioningStatus)

    let mut c = false;
    if !state.bdbNodeIsOnANetwork {
        match state.device_type {
            DeviceType::EndDevice => (),
            _ => state = network_formation(state)
        }
    }
    if state.bdbCommissioningMode.contains(CommissioningModeFlag::FindingAndBinding) {
        MachineState::Commissioning(state, CommissioningMode::FindingAndBinding)
    } else {
        MachineState::CommissioningDone(state)
    }
}

fn network_formation(state: State) -> State {
    // TODO: Perform network formation
    state
}

pub fn try_finding_and_binding(state: State) -> MachineState {
    // TODO: - FindingAndBinding init
    //       - Add actual response to the state struct (bdbCommissioningStatus)
    MachineState::CommissioningDone(state)
}