use super::Data;

#[repr(packed)]  // required for network exchange
#[derive(Copy, Clone, Debug)]
/// The Header part of the Box.
///
/// It can be copied or cloned cheaply.
/// # Fields
/// - data_size - unsigned 64 bit value used to inform the reader about the size of incoming Data
/// - data_type - unsigned 16 bit value used to inform the reader about the type of incoming Data
/// - queue - unsigned 8 bit value used to inform the server to which queue use to handle the Box
pub struct Header {
    data_size: u64,  // cannot access due to packed representation
    data_type: u16,
    queue: u8,
}

impl Header {
    /// Size of the Header in bytes.
    pub const SIZE: usize = std::mem::size_of::<Header>();

    // getters needed due to packed representation of the struct

    /// Returns the data_size.
    pub fn get_data_size(&self) -> u64 {
        self.data_size
    }

    /// Returns the data_type.
    pub fn get_data_type(&self) -> u16 {
        self.data_type
    }

    /// Returns the queue.
    pub fn get_data_queue(&self) -> u8 {
        self.queue
    }

    /// Creates a new Header based on a given data.
    /// # Fields
    /// - data - the Data to build the Header for
    /// - data_type - type of the Data
    /// - queue - the queue number
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

    /// Transforms the Header int an array.
    pub fn into_array(self) -> [u8; Header::SIZE] {
        self.into()
    }

    /// Transforms the array back into the Header.
    ///
    /// ## Waring
    /// The header will be built from the array no matter what values it holds
    pub fn from_array(bytes: [u8; Header::SIZE]) -> Self {
        Self::from(bytes)
    }
}

/// Same as `Header::into_array`
impl Into<[u8; Header::SIZE]> for Header {
    fn into(self) -> [u8; Header::SIZE] {
        unsafe { std::mem::transmute(self) }
    }
}

/// Same as `Header::from_array`
impl From<[u8; Header::SIZE]> for Header {
    fn from(bytes: [u8; Header::SIZE]) -> Self {
        unsafe { std::mem::transmute(bytes) }
    }
}
