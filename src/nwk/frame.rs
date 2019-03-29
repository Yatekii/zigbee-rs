use crate::serde::Serde;

pub enum SerdeError {
    NotEnoughSpace,
    WrongNumberOfBytes,
    UnknownFrameType,
    BrokenRelayList,
}

/// 3.3.1.1.1 Frame Type Sub-Field
#[derive(Copy, Clone)]
pub enum FrameType {
    Data = 0b00,
    NWKCommand = 0b01,
    InterPan = 0b11,
}

impl Serde<FrameType, SerdeError> for FrameType {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        if data.len() == 1 {
            Ok(*self as u8)
        } else {
            Err(SerdeError::NotEnoughSpace)
        }
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        if data.len() != 1 {
            Err(SerdeError::WrongNumberOfBytes)
        } else {
            let frame_type = data[0] & 0b11;
            if frame_type == FrameType::Data as u8 {
                Ok(FrameType::Data)
            } else if frame_type == FrameType::InterPan as u8 {
                Ok(FrameType::InterPan)
            } else if frame_type == FrameType::NWKCommand as u8 {
                Ok(FrameType::NWKCommand)
            } else {
                Err(SerdeError::UnknownFrameType)
            }
        }  
    }
}

/// 3.3.1.1.3 Discover Route Sub-Field
#[derive(Copy, Clone)]
pub enum DiscoverRoute {
    SurpressDiscovery = 0b00,
    EnableDiscovery = 0b01,
}

impl Serde<DiscoverRoute, SerdeError> for DiscoverRoute {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        if data.len() == 1 {
            Ok(*self as u8)
        } else {
            Err(SerdeError::NotEnoughSpace)
        }
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        if data.len() != 1 {
            Err(SerdeError::WrongNumberOfBytes)
        } else {
            let frame_type = (data[0] >> 6) & 0b11;
            if frame_type == DiscoverRoute::SurpressDiscovery as u8 {
                Ok(DiscoverRoute::SurpressDiscovery)
            } else if frame_type == DiscoverRoute::SurpressDiscovery as u8 {
                Ok(DiscoverRoute::SurpressDiscovery)
            } else {
                Err(SerdeError::UnknownFrameType)
            }
        }  
    }
}

/// 3.3.1.1 Frame Control Field
pub struct FrameControl {
    // 3.3.1.1.1 Frame Type Sub-Field
    pub frame_type: FrameType,
    // 3.3.1.1.2 Protocol Version Sub-Field
    pub protocol_version: u8,
    /// 3.3.1.1.3 Discover Route Sub-Field
    pub discover_route: DiscoverRoute,
    // 3.3.1.1.4 Multicast Flag Sub-Field
    multicast: bool,
    // 3.3.1.1.5 Security Sub-Field
    security_enabled: bool,
    // 3.3.1.1.6 Source Route Sub-Field
    contains_source_route_frame: bool,
    // 3.3.1.1.7    Destination IEEE Address Sub-Field
    contains_destination_ieee_address: bool,
    // 3.3.1.1.8    Source IEEE Address Sub-Field
    contains_source_ieee_address: bool,
}

impl Serde<FrameControl, SerdeError> for FrameControl {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        if data.len() != 2 {
            Err(SerdeError::WrongNumberOfBytes)
        } else {
            let mut frame_type = 0;
            let mut discover_route = 0;
            self.frame_type.serialize(std::slice::from_mut(&mut frame_type))?;
            self.discover_route.serialize(std::slice::from_mut(&mut discover_route))?;

            data[0] = frame_type
                    & (self.protocol_version << 2)
                    & discover_route;
            data[1] = self.multicast as u8
                    & ((self.security_enabled as u8) << 1)
                    & ((self.contains_source_route_frame as u8) << 2)
                    & ((self.contains_destination_ieee_address as u8) << 3)
                    & ((self.contains_source_ieee_address as u8) << 4);
            Ok(2)
        }
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        if data.len() != 2 {
            Err(SerdeError::WrongNumberOfBytes)
        } else {
            Ok(Self {
                frame_type: FrameType::deserialize(&data[0..1])?,
                protocol_version: (data[0] >> 2) & 0b1111,
                discover_route: DiscoverRoute::deserialize(&data[0..1])?,
                multicast: if (data[1] >> 0) & 0b1 == 1 { true } else { false },
                security_enabled: if (data[1] >> 1) & 0b1 == 1 { true } else { false },
                contains_source_route_frame: if (data[1] >> 2) & 0b1 == 1 { true } else { false },
                contains_destination_ieee_address: if (data[1] >> 3) & 0b1 == 1 { true } else { false },
                contains_source_ieee_address: if (data[1] >> 4) & 0b1 == 1 { true } else { false },
            })
        }
    }
}

pub struct SourceRouteFrame {
    relay_index: u8,
    relay_list: Vec<[u8; 2]>
}

impl SourceRouteFrame {
    pub fn new(relay_list: Vec<[u8; 2]>) -> Self {
        if relay_list.len() == 0 {
            panic!("Relay list cannot be of length 0.");
        }
        Self {
            relay_index: relay_list.len() as u8 - 1,
            relay_list
        }
    }

    pub fn len(&self) -> u8 {
        self.relay_list.len() as u8
    }

    pub fn get_index(&self) -> u8 {
        self.relay_index
    }

    pub fn decrement_index(&mut self) {
        self.relay_index -= 1;
    }
}

impl Serde<SourceRouteFrame, SerdeError> for SourceRouteFrame {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        if data.len() < 2 + self.relay_list.len() * 2 {
            Err(SerdeError::WrongNumberOfBytes)
        } else {
            data[0] = self.relay_list.len() as u8;
            data[1] = self.relay_index;
            for (i, address) in self.relay_list.iter().enumerate() {
                data[(i + 1) * 2    ] = address[0];
                data[(i + 1) * 2 + 1] = address[1];
            }
            Ok(2 + self.relay_list.len() as u8 * 2)
        }
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        if data[0] == 0 || data[0] != (data.len() as u8 - 2) / 2 || data[1] >= data[0] {
            Err(SerdeError::BrokenRelayList)
        } else {
            let mut relay_list = vec![[0; 2]; data[0] as usize];
            for (i, chunk) in data[2..].chunks(2).enumerate() {
                let mut source_address = [0; 2];
                relay_list[i].clone_from_slice(chunk);
            }
            Ok(Self {
                relay_index: data[0],
                relay_list
            })
        }
    }
}

pub struct NPDUFrame {
    pub control: FrameControl,
    pub destination_address: [u8; 2],
    pub source_address: [u8; 2],
    pub radius: u8,
    pub sequence_number: u8,
    pub destination_ieee_address: Option<[u8; 8]>,
    pub source_ieee_address: Option<[u8; 8]>,
    pub multicast_control: Option<u8>,
    pub source_route_frame: Option<SourceRouteFrame>,
    pub payload: Option<[u8; 42]>,
}

impl Serde<NPDUFrame, SerdeError> for NPDUFrame {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        Err(SerdeError::NotEnoughSpace)
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        const MIN_NUM_BYTES: usize = 8;

        if data.len() < MIN_NUM_BYTES {
            Err(SerdeError::WrongNumberOfBytes)
        } else {
            let mut destination_address = [0; 2];
            let mut source_address = [0; 2];
            destination_address.clone_from_slice(&data[2..4]);
            source_address.clone_from_slice(&data[4..6]);
            let mut frame = NPDUFrame {
                control: FrameControl::deserialize(&data[0..2])?,
                destination_address: destination_address,
                source_address: source_address,
                radius: data[6],
                sequence_number: data[7],
                destination_ieee_address: None,
                source_ieee_address: None,
                multicast_control: None,
                source_route_frame: None,
                payload: None,
            };

            let mut total_length = MIN_NUM_BYTES;

            if frame.control.contains_destination_ieee_address {
                let mut destination_ieee_address = [0; 8];
                destination_ieee_address.clone_from_slice(&data[total_length..total_length + 8]);
                frame.destination_ieee_address = Some(destination_ieee_address);
                total_length += 8;
            }

            if frame.control.contains_source_ieee_address {
                let mut source_ieee_address = [0; 8];
                source_ieee_address.clone_from_slice(&data[total_length..total_length + 8]);
                frame.source_ieee_address = Some(source_ieee_address);
                total_length += 8;
            }

            if frame.control.multicast {
                frame.multicast_control = Some(data[total_length]);
                total_length += 1;
            }

            if frame.control.contains_source_route_frame {
                let data_end = total_length + 2 + data[total_length] as usize;
                frame.source_route_frame = Some(SourceRouteFrame::deserialize(&data[total_length..data_end])?);
                total_length += data[total_length] as usize;
                total_length += 2;
            }

            // TODO: Handle payload

            if data.len() < total_length as usize {
                Err(SerdeError::WrongNumberOfBytes)
            } else {
                Ok(frame)
            }
        }
    }
}