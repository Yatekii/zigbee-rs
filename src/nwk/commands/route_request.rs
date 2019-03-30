use crate::nwk::address::{
    AddressType,
    Address,
    IEEEAddress,
    GroupIdentifier,
};
use crate::serde::Serde;
use crate::nwk::frame::SerdeError;

#[derive(Copy, Clone)]
pub enum ManyToOne {
    No = 0x0,
    SupportForRRTable = 0x1,
    NoSupportForRRTable = 0x2,
}

impl Serde<ManyToOne, SerdeError> for ManyToOne {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        if data.len() == 1 {
            data[0] = (*self as u8) << 3;
            Ok(1)
        } else {
            Err(SerdeError::NotEnoughSpace)
        }
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        if data.len() != 1 {
            Err(SerdeError::WrongNumberOfBytes)
        } else {
            let frame_type = (data[0] >> 3) & 0b11;
            if frame_type == ManyToOne::No as u8 {
                Ok(ManyToOne::No)
            } else if frame_type == ManyToOne::SupportForRRTable as u8 {
                Ok(ManyToOne::SupportForRRTable)
            } else if frame_type == ManyToOne::NoSupportForRRTable as u8 {
                Ok(ManyToOne::NoSupportForRRTable)
            } else {
                Err(SerdeError::UnknownFrameType)
            }
        }
    }
}

pub struct CommandOptions {
    many_to_one: ManyToOne,
    contains_destination_ieee_address: bool,
    is_multicast: bool,
}

impl Serde<CommandOptions, SerdeError> for CommandOptions {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        self.many_to_one.serialize(&mut data[1..2])?;
        data[1] =& ((self.contains_destination_ieee_address as u8) << 5)
                 & ((self.is_multicast as u8) << 6);
        Ok(1)
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        if data.len() > 0 {
            Ok(CommandOptions {
                many_to_one: ManyToOne::deserialize(&data[0..1])?,
                contains_destination_ieee_address: if (data[1] >> 5) & 0b1 == 1 { true } else { false },
                is_multicast: if (data[1] >> 6) & 0b1 == 1 { true } else { false },
            })
        } else {
            Err(SerdeError::WrongNumberOfBytes)
        }
    }
}

pub struct RouteRequest {
    command_options: CommandOptions,
    route_request_identifier: u8,
    destination_address: AddressType,
    path_cost: u8,
    destination_ieee_address: Option<IEEEAddress>,
}

impl Serde<RouteRequest, SerdeError> for RouteRequest {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        Ok(0)
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        if data.len() > 0 {
            let command_options = CommandOptions::deserialize(&data[0..1])?;
            let contains_destination_ieee_address = command_options.contains_destination_ieee_address;
            let is_multicast = command_options.is_multicast;
            if (contains_destination_ieee_address && data.len() == 13) || data.len() == 5 {
                Ok(RouteRequest {
                    command_options: command_options,
                    route_request_identifier: data[1],
                    destination_address: if is_multicast {
                        AddressType::Multicast(GroupIdentifier::deserialize(&data[2..4])?)
                    } else {
                        AddressType::Singlecast(Address::deserialize(&data[2..4])?)
                    },
                    path_cost: data[4],
                    destination_ieee_address: if contains_destination_ieee_address {
                        Some(IEEEAddress::deserialize(&data[5..13])?)
                    } else {
                        None
                    },
                })
            } else {
                Err(SerdeError::WrongNumberOfBytes)
            }
        } else {
            Err(SerdeError::WrongNumberOfBytes)
        }
    }
}