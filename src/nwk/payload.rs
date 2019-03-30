use crate::nwk::commands::route_request::RouteRequest;
use crate::serde::Serde;
use crate::nwk::frame::SerdeError;

/// 3.3.2.1  Data Frame Format
pub struct DataFrame {

}

/// Table 3.40   NWK Command Frames
pub enum NWKCommandFrame {
    // 0x01
    RouteRequest(RouteRequest),
    RouteReply,
    NetworkStatus,
    Leave,
    RouteRecord,
    RejoinRequest,
    RejoinResponse,
    LinkStatus,
    NetworkReport,
    NetworkUpdate,
}

/// 3.3.2 Format of Individual Frame Types
pub enum Payload {
    Data(DataFrame),
    NWKCommand(NWKCommandFrame),
    InterPan,
}

impl Payload {
    pub fn new_nwk_command(data: &[u8]) -> Result<Self, SerdeError> {
        if data[0] == 0x01 {
            Ok(Payload::NWKCommand(NWKCommandFrame::RouteRequest(RouteRequest::deserialize(&data[1..])?)))
        } else {
            Err(SerdeError::UnknownNWKCommand)
        }
    }
}

// impl Serde<Payload, SerdeError> for Payload {
//     fn serialize(&self, data: &mut [u8]) -> Result<u8, SerdeError> {
//         Ok(0)
//     }
    
//     fn deserialize(data: &[u8]) -> Result<Self, SerdeError> {
//         // TODO: How to properly deserialize this if we just know the payload
//         //       but not the payload type (parsed outside)?
//         Ok(Payload::Data(DataFrame {}))
//     }
// }