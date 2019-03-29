use bitflags::bitflags;

pub struct State {
    pub device_type: DeviceType,
    pub bdbNodeIsOnANetwork: bool,
    pub bdbCommissioningCapability: CommissioningModeFlag,
    pub bdbCommissioningMode: CommissioningModeFlag,
    pub bdbcTLPrimaryChannelSet: u8,
    pub bdbCommitssioningStatus: CommissioningStatus,

}

impl State {
    pub fn new() -> Self {
        Self {
            device_type: DeviceType::EndDevice,
            bdbNodeIsOnANetwork: false,
            bdbCommissioningCapability: CommissioningModeFlag::None,
            bdbCommissioningMode: CommissioningModeFlag::None,
            bdbcTLPrimaryChannelSet: 0,
            bdbCommitssioningStatus: CommissioningStatus::SUCCESS,
        }
    }
}

pub enum DeviceType {
    Coordinator = 0b000,
    Router = 0b001,
    EndDevice = 0b010,
}

pub enum CommissioningMode {
    Touchlink         = 0b0000001,
    NetworkSteering   = 0b0000010,
    NetworkFormation  = 0b0000100,
    FindingAndBinding = 0b0001000,
}

impl From<CommissioningMode> for CommissioningModeFlag {
    fn from(mode: CommissioningMode) -> Self {
        CommissioningModeFlag::from_bits(mode as u8).expect("UB encountered. This is a bug and should be reported!")
    }
}

bitflags! {
    pub struct CommissioningModeFlag: u8 {
        const Touchlink         = 0b0000001;
        const NetworkSteering   = 0b0000010;
        const NetworkFormation  = 0b0000100;
        const FindingAndBinding = 0b0001000;
        const None              = 0b0000000;
    }
}

pub enum CommissioningStatus {
    SUCCESS = 0,
    IN_PROGRESS = 1,
    NOT_AA_CAPABLE = 2,
    NO_NETWORK = 3,
    TARGET_FAILURE = 4,
    FORMATION_FAILURE = 5,
    NO_IDENTIFY_QUERY = 6,
    BINDING_TABLE_FULL = 7,
    NO_SCAN_RESPONSE = 8,
    NOT_PERMITTED = 9,
    TCLK_EX_FAILURE = 10,
}