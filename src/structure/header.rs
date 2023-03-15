use crate::structure::data::Data;


#[repr(packed)]  // required for network exchange
#[derive(Copy, Clone, Debug)]
pub struct Header {
    data_size: u64,  // cannot access due to packed representation
    data_type: u16,
    queue: u8,
}

impl Header {
    /// Size of the Header in bytes.
    pub const SIZE: usize = std::mem::size_of::<Header>();

    pub fn get_data_size(&self) -> u64 {
        self.data_size
    }

    pub fn get_data_type(&self) -> u16 {
        self.data_type
    }

    pub fn get_data_queue(&self) -> u8 {
        self.queue
    }

    /// Creates a new Header based on a given data.
    pub(crate) fn build_for_data<'a>(
        data: impl AsRef<Data<'a>>,
        data_type: impl Into<u16>,
        queue: u8
    ) -> Self
    {
        Self {
            data_size: data.as_ref().len() as u64,
            data_type: data_type.into(),
            queue,
        }
    }

    pub fn into_array(self) -> [u8; Header::SIZE] {
        self.into()
    }

    pub fn from_array(bytes: [u8; Header::SIZE]) -> Self {
        Self::from(bytes)
    }
}

impl Into<[u8; Header::SIZE]> for Header {
    fn into(self) -> [u8; Header::SIZE] {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<[u8; Header::SIZE]> for Header {
    fn from(bytes: [u8; Header::SIZE]) -> Self {
        unsafe { std::mem::transmute(bytes) }
    }
}
