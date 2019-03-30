use crate::serde::Serde;
use crate::nwk::frame::SerdeError;

/// 3.3.2.1  Data Frame Format
pub struct DataFrame {

}

/// 3.3.2.2  NWK Command Frame Format
pub struct NWKCommandFrame {

}

/// 3.3.2 Format of Individual Frame Types
pub enum Payload {
    Data(DataFrame),
    NWKCommand(NWKCommandFrame),
    InterPan,
}

impl Serde<Payload, SerdeError> for Payload {
    fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
        Ok(0)
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
        // TODO: How to properly deserialize this if we just know the payload
        //       but not the payload type (parsed outside)?
        Ok(Payload::Data(DataFrame {}))
    }
}