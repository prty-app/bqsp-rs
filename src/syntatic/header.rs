use std::mem::size_of;
use super::Data;

pub type HeaderDataSize = u32;
pub type HeaderDataType = u16;
pub type HeaderQueue = u8;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The Header part of the Box.
///
/// It can be copied or cloned cheaply.
/// # Fields
/// - data_size - unsigned 32 bit value used to inform the reader about the size of incoming Data
/// - data_type - unsigned 16 bit value used to inform the reader about the type of incoming Data
/// - queue - unsigned 8 bit value used to inform the server to which queue use to handle the Box
pub struct Header {
    data_size: HeaderDataSize,
    data_type: HeaderDataType,
    queue: HeaderQueue,
}

impl Header {
    /// Size of the Header in bytes.
    pub const SIZE: usize = size_of::<HeaderDataSize>() + size_of::<HeaderDataType>() + size_of::<HeaderQueue>();

    // getters needed due to packed representation of the struct

    /// Returns the data_size.
    pub fn get_data_size(&self) -> HeaderDataSize {
        self.data_size
    }

    /// Returns the data_type.
    pub fn get_data_type(&self) -> HeaderDataType {
        self.data_type
    }

    /// Returns the queue.
    pub fn get_data_queue(&self) -> HeaderQueue {
        self.queue
    }

    /// Creates a new Header based on a given data.
    /// # Fields
    /// - data - the Data to build the Header for
    /// - data_type - type of the Data
    /// - queue - the queue number
    pub(crate) fn build_for_data<'a>(
        data: impl AsRef<Data<'a>>,
        data_type: impl Into<HeaderDataType>,
        queue: HeaderQueue
    ) -> Self
    {
        Self {
            data_size: data.as_ref().len() as HeaderDataSize,
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
        let mut buffer = [0x00_u8; Header::SIZE];
        let buffer_ptr = buffer.as_mut_ptr();

        let data_size_ptr = self.data_size.to_le_bytes().as_ptr();
        let data_type_ptr = self.data_type.to_le_bytes().as_ptr();
        let queue_ptr = self.queue.to_le_bytes().as_ptr();

        let mut cursor = 0;

        unsafe {
            copy_bytes_to(
                data_size_ptr,
                buffer_ptr,
                size_of::<HeaderDataSize>(),
                &mut cursor,
            );

            copy_bytes_to(
                data_type_ptr,
                buffer_ptr,
                size_of::<HeaderDataType>(),
                &mut cursor,
            );

            copy_bytes_to(
                queue_ptr,
                buffer_ptr,
                size_of::<HeaderQueue>(),
                &mut cursor,
            );
        }

        buffer
    }
}

/// Same as `Header::from_array`
impl From<[u8; Header::SIZE]> for Header {
    fn from(bytes: [u8; Header::SIZE]) -> Self {
        let mut buffer_data_size = [0x00_u8; size_of::<HeaderDataSize>()];
        let mut buffer_data_type = [0x00_u8; size_of::<HeaderDataType>()];
        let mut buffer_queue = [0x00_u8; size_of::<HeaderQueue>()];

        let bytes_ptr = bytes.as_ptr();

        let mut cursor = 0;

        unsafe {
            copy_bytes_from(
                bytes_ptr,
                buffer_data_size.as_mut_ptr(),
                size_of::<HeaderDataSize>(),
                &mut cursor,
            );

            copy_bytes_from(
                bytes_ptr,
                buffer_data_type.as_mut_ptr(),
                size_of::<HeaderDataType>(),
                &mut cursor,
            );

            copy_bytes_from(
                bytes_ptr,
                buffer_queue.as_mut_ptr(),
                size_of::<HeaderQueue>(),
                &mut cursor,
            );
        }

        Self {
            data_size: HeaderDataSize::from_le_bytes(buffer_data_size),
            data_type: HeaderDataType::from_le_bytes(buffer_data_type),
            queue: HeaderQueue::from_le_bytes(buffer_queue),
        }
    }
}

/// Copies bytes into one specific buffer.
///
/// # Args
/// - src - pointer to a byte array from which bytes will be copied
/// - dst - pointer to a buffer
/// - count - amount of bytes to copy from src (should be size_of src)
/// - cursor - cursor over the buffer
unsafe fn copy_bytes_to(src: *const u8, dst: *mut u8, count: usize, cursor: &mut usize) {
    std::ptr::copy(
        src,
        dst.add(*cursor),
        count
    );

    *cursor += count;
}

/// Copies bytes from one specific array.
///
/// # Args
/// - src - pointer to an array
/// - dst - pointer to a buffer into which bytes will be copied to
/// - count - amount of bytes to copy from src (should be size_of dst)
/// - cursor - cursor over the array
unsafe fn copy_bytes_from(src: *const u8, dst: *mut u8, count: usize, cursor: &mut usize) {
    std::ptr::copy(
        src.add(*cursor),
        dst,
        count
    );

    *cursor += count;
}
