use core::borrow::BorrowMut;
use core::borrow::Borrow;
use crate::serde::Serde;
use crate::nwk::frame::SerdeError;

/// 3.3.2.1  Data Frame Format
pub struct DataFrame {

}

pub struct RouteRequest<T> {
    data: T,
    // command_options: u8,
    // route_request_identifier: u8,
    // destination_address: [u8; 2],
    // path_cost: u8,
    // destination_ieee_address: [u8; 8],
}

impl<T> RouteRequest<T> {
    pub fn new(data: T) -> RouteRequest<T> {
        RouteRequest {
            data,
        }
    }
}

impl<T> RouteRequest<T> where T: Borrow<[u8]> {
    pub fn get_command_options(&self) -> u8 {
        self.data.borrow()[0]
    }

    pub fn get_route_request_identifier(&self) -> u8 {
        self.data.borrow()[1]
    }

    pub fn get_destination_address(&self) -> &[u8] {
        &self.data.borrow()[2..4]
    }

    pub fn get_path_cost(&self) -> u8 {
        self.data.borrow()[4]
    }

    pub fn get_destination_ieee_address(&self) -> &[u8] {
        &self.data.borrow()[5..13]
    }
}

impl<T> RouteRequest<T> where T: BorrowMut<[u8]> {
    pub fn set_command_options(&mut self, data: u8) {
        self.data.borrow_mut()[0] = data;
    }

    pub fn set_route_request_identifier(&mut self, data: u8) {
        self.data.borrow_mut()[1] = data;
    }

    pub fn set_destination_address(&mut self, data: &[u8]) {
        self.data.borrow_mut()[2..4].clone_from_slice(data);
    }

    pub fn set_path_cost(&mut self, data: u8) {
        self.data.borrow_mut()[4] = data;
    }

    pub fn set_destination_ieee_address(&mut self, data: &[u8]) {
        self.data.borrow_mut()[5..13].clone_from_slice(data);
    }
}

/// Table 3.40   NWK Command Frames
pub enum NWKCommandFrame<T> {
    // 0x01
    RouteRequest(RouteRequest<T>),
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
pub enum Payload<T> {
    Data(DataFrame),
    NWKCommand(NWKCommandFrame<T>),
    InterPan,
}

impl<'a> Payload<&'a [u8]> {
    pub fn new(data: &'a [u8]) -> Self {
        Payload::NWKCommand(NWKCommandFrame::RouteRequest(RouteRequest::new(data)))
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