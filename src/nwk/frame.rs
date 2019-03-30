use crate::nwk::payload::Payload;
use crate::serde::Serde;

pub enum SerdeError {
    NotEnoughSpace,
    WrongNumberOfBytes,
    UnknownFrameType,
    BrokenRelayList,
}

/// 3.3.1.1.1 Frame Type Sub-Field
#[derive(Copy, Clone)]
pub enum FrameTypeEnum {
    Data = 0b00,
    NWKCommand = 0b01,
    InterPan = 0b11,
}

impl Serde<FrameTypeEnum, SerdeError> for FrameTypeEnum {
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
            if frame_type == FrameTypeEnum::Data as u8 {
                Ok(FrameTypeEnum::Data)
            } else if frame_type == FrameTypeEnum::InterPan as u8 {
                Ok(FrameTypeEnum::InterPan)
            } else if frame_type == FrameTypeEnum::NWKCommand as u8 {
                Ok(FrameTypeEnum::NWKCommand)
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
#[derive(Copy, Clone)]
pub struct FrameControl {
    // 3.3.1.1.1 Frame Type Sub-Field
    pub frame_type: FrameTypeEnum,
    // 3.3.1.1.2 Protocol Version Sub-Field
    pub protocol_version: u8,
    // 3.3.1.1.3 Discover Route Sub-Field
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
                frame_type: FrameTypeEnum::deserialize(&data[0..1])?,
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

/// 3.3.1.8.1 Multicast Mode Sub-Field
#[derive(Copy, Clone)]
pub enum MulticastMode {
    NonmemberMode = 0b00,
    MemberMode = 0b01,
}

impl Serde<MulticastMode, SerdeError> for MulticastMode {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        unimplemented!();
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        if data.len() != 1 {
            Err(SerdeError::WrongNumberOfBytes)
        } else {
            let frame_type = data[0] & 0b11;
            if frame_type == MulticastMode::NonmemberMode as u8 {
                Ok(MulticastMode::NonmemberMode)
            } else if frame_type == MulticastMode::MemberMode as u8 {
                Ok(MulticastMode::MemberMode)
            } else {
                Err(SerdeError::UnknownFrameType)
            }
        }  
    }
}

/// 3.3.1.8 Multicast Control Field
#[derive(Copy, Clone)]
pub struct MulticastControl {
    // 3.3.1.8.1 Multicast Mode Sub-Field
    pub multicast_mode: MulticastMode,
    // 3.3.1.8.2 NonmemberRadius Sub-Field
    pub nonmember_radius: u8,
    // 3.3.1.8.3 MaxNonmemberRadius Sub-Field
    pub max_nonmember_radius: u8,
}

impl Serde<MulticastControl, SerdeError> for MulticastControl {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        if data.len() != 2 {
            Err(SerdeError::WrongNumberOfBytes)
        } else {
            data[0] = self.multicast_mode as u8
                    & (self.nonmember_radius << 2)
                    & (self.max_nonmember_radius << 5);
            Ok(1)
        }
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        if data.len() != 2 {
            Err(SerdeError::WrongNumberOfBytes)
        } else {
            Ok(Self {
                multicast_mode: MulticastMode::deserialize(&data[0..1])?,
                nonmember_radius: (data[0] >> 2) & 0b111,
                max_nonmember_radius: (data[0] >> 5) & 0b111,
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
    pub payload: Payload,
}

const MIN_NUM_BYTES: usize = 8;

impl Serde<NPDUFrame, SerdeError> for NPDUFrame {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        let mut control = self.control;
        let mut total_length = MIN_NUM_BYTES;

        data[2..4].clone_from_slice(&self.destination_address);
        data[4..6].clone_from_slice(&self.source_address);
        data[6] = self.radius;
        data[7] = self.sequence_number;

        control.contains_destination_ieee_address =
            if let Some(v) = self.destination_ieee_address {
                if data.len() > total_length + 8 {
                    data[total_length..total_length + 8].clone_from_slice(&v);
                    total_length += 8;
                    true
                } else {
                    return Err(SerdeError::NotEnoughSpace)
                }
            } else {
                false
            };

        control.contains_source_ieee_address =
            if let Some(v) = self.source_ieee_address {
                if data.len() > total_length + 8 {
                    data[total_length..total_length + 8].clone_from_slice(&v);
                    total_length += 8;
                    true
                } else {
                    return Err(SerdeError::NotEnoughSpace)
                }
            } else {
                false
            };

        control.multicast =
            if let Some(v) = self.multicast_control {
                if data.len() > total_length + 8 {
                    data[total_length] = v;
                    total_length += 1;
                    true
                } else {
                    return Err(SerdeError::NotEnoughSpace)
                }
            } else {
                false
            };

        control.contains_source_route_frame =
            if let Some(v) = &self.source_route_frame {
                if data.len() > total_length + 8 {
                    let length = v.serialize(&mut data[total_length..total_length + 8])?;
                    total_length += length as usize;
                    true
                } else {
                    return Err(SerdeError::NotEnoughSpace)
                }
            } else {
                false
            };

        total_length += self.payload.serialize(&mut data[total_length..])? as usize;

        control.serialize(&mut data[0..2])?;

        Ok(total_length as u8)
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        if data.len() < MIN_NUM_BYTES {
            Err(SerdeError::WrongNumberOfBytes)
        } else {
            let mut destination_address = [0; 2];
            let mut source_address = [0; 2];
            destination_address.clone_from_slice(&data[2..4]);
            source_address.clone_from_slice(&data[4..6]);

            let frame_control = FrameControl::deserialize(&data[0..2])?;

            let mut total_length = MIN_NUM_BYTES;

            let destination_ieee_address =
                if frame_control.contains_destination_ieee_address {
                    let mut destination_ieee_address = [0; 8];
                    destination_ieee_address.clone_from_slice(&data[total_length..total_length + 8]); 
                    total_length += 8;
                    Some(destination_ieee_address)
                } else {
                    None
                };

            let source_ieee_address =
                if frame_control.contains_source_ieee_address {
                    let mut source_ieee_address = [0; 8];
                    source_ieee_address.clone_from_slice(&data[total_length..total_length + 8]);
                    total_length += 8;
                    Some(source_ieee_address)
                } else {
                    None
                };

            let multicast_control =
                if frame_control.multicast {
                    let multicast_control = Some(data[total_length]);
                    total_length += 1;
                    multicast_control
                } else {
                    None
                };

            let source_route_frame =
                if frame_control.contains_source_route_frame {
                    let data_end = total_length + 2 + data[total_length] as usize;
                    let source_route_frame = Some(SourceRouteFrame::deserialize(&data[total_length..data_end])?);
                    total_length += data[total_length] as usize;
                    total_length += 2;
                    source_route_frame
                } else {
                    None
                };

            if data.len() < total_length as usize {
                Err(SerdeError::WrongNumberOfBytes)
            } else {
                Ok(NPDUFrame {
                    control: frame_control,
                    destination_address: destination_address,
                    source_address: source_address,
                    radius: data[6],
                    sequence_number: data[7],
                    destination_ieee_address: destination_ieee_address,
                    source_ieee_address: source_ieee_address,
                    multicast_control: multicast_control,
                    source_route_frame: source_route_frame,
                    payload: Payload::deserialize(&data[total_length..])?,
                })
            }
        }
    }
}