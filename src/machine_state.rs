use crate::state::{
    CommissioningMode,
    State,
};

pub enum MachineState {
    RestorePersistentData,
    AttemptRejoin(State),
    BroadcastDeviceAnnounce(State),
    InitDone(State),
    CommissioningBegin(State),
    Commissioning(State, CommissioningMode),
    CommissioningDone(State),
}