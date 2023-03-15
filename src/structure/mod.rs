pub mod header;
pub mod data;

use header::*;
use data::*;


pub struct BoxPack<'d> {
    pub header: Header,
    pub data: Data<'d>,
}

impl<'d> BoxPack<'d> {
    pub fn new(data: Data<'d>, data_type: impl Into<u16>, queue: u8) -> Self {
        Self {
            header: Header::build_for_data(&data, data_type, queue),
            data,
        }
    }
}
