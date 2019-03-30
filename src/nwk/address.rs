use crate::nwk::frame::SerdeError;
use crate::serde::Serde;

pub struct Address([u8; 2]);

impl Serde<Address, SerdeError> for Address {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        if data.len() > 1 {
            data.clone_from_slice(&self.0);
            Ok(2)
        } else {
            Err(SerdeError::NotEnoughSpace)
        }
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        if data.len() == 2 {
            let mut address = Address([0; 2]);
            address.0.clone_from_slice(&data);
            Ok(address)
        } else {
            Err(SerdeError::WrongNumberOfBytes)
        }
    }
}


pub struct IEEEAddress([u8; 8]);

impl Serde<IEEEAddress, SerdeError> for IEEEAddress {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        if data.len() > 7 {
            data.clone_from_slice(&self.0);
            Ok(8)
        } else {
            Err(SerdeError::NotEnoughSpace)
        }
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        if data.len() == 8 {
            let mut address = IEEEAddress([0; 8]);
            address.0.clone_from_slice(&data);
            Ok(address)
        } else {
            Err(SerdeError::WrongNumberOfBytes)
        }
    }
}

pub struct GroupIdentifier([u8; 2]);

impl Serde<GroupIdentifier, SerdeError> for GroupIdentifier {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        if data.len() > 1 {
            data.clone_from_slice(&self.0);
            Ok(2)
        } else {
            Err(SerdeError::NotEnoughSpace)
        }
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        if data.len() == 2 {
            let mut group_id = GroupIdentifier([0; 2]);
            group_id.0.clone_from_slice(&data);
            Ok(group_id)
        } else {
            Err(SerdeError::WrongNumberOfBytes)
        }
    }
}

pub enum AddressType {
    Singlecast(Address),
    Multicast(GroupIdentifier),
}