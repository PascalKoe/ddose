use embedded_hal::can;

/// Holds a complete CAN frame including the header.
pub struct CanFrame(libc::can_frame);

impl CanFrame {
    /// Creates a new CAN frame from an Linux CAN frame
    pub fn from_inner(can_frame: libc::can_frame) -> Self {
        Self(can_frame)
    }
    /// Returns the inner representation of the CAN frame
    pub fn inner(&self) -> &libc::can_frame {
        &self.0
    }
}

impl embedded_hal::can::Frame for CanFrame {
    fn new(id: impl Into<embedded_hal::can::Id>, data: &[u8]) -> Option<Self> {
        // According to the trait defintion `None` shall be returned when
        // the data slice is too long
        if data.len() > 8 {
            return None;
        }

        // We need to know if we deal with an EFF because we need to set
        // the bit in the can_id field of the C representation
        let (raw_id, eff_flag) = match id.into() {
            can::Id::Extended(extended_id) => (extended_id.as_raw(), libc::CAN_EFF_FLAG),
            can::Id::Standard(standard_id) => (standard_id.as_raw() as u32, 0),
        };

        // UNSAFE: The C struct layout needs to be zeroed in order for the padding
        // and reserved values to be valid
        let mut c_can_frame: libc::can_frame = unsafe { std::mem::zeroed() };

        // The EFF flag is part of the CAN id field, so we need to set
        // it according to the used id type
        c_can_frame.can_id = raw_id | eff_flag;

        c_can_frame.can_dlc = data.len() as u8;

        // We allready know, that the length of data is at most 8 bytes
        // so there is no chance for out of bounds errors
        c_can_frame.data[..data.len()].copy_from_slice(data);

        Some(Self(c_can_frame))
    }

    fn new_remote(id: impl Into<embedded_hal::can::Id>, dlc: usize) -> Option<Self> {
        // According to the trait defintion `None` shall be returned when
        // the DLC is invalid
        if dlc > 8 {
            return None;
        }

        // We need to know if we deal with an EFF because we need to set
        // the bit in the can_id field of the C representation
        let (raw_id, eff_flag) = match id.into() {
            can::Id::Extended(extended_id) => (extended_id.as_raw(), libc::CAN_EFF_FLAG),
            can::Id::Standard(standard_id) => (standard_id.as_raw() as u32, 0),
        };

        // UNSAFE: The C struct layout needs to be zeroed in order for the padding
        // and reserved values to be valid
        let mut c_can_frame: libc::can_frame = unsafe { std::mem::zeroed() };

        // The EFF and RTR flag is part of the CAN id field, so we need to set them
        c_can_frame.can_id = raw_id | eff_flag | libc::CAN_RTR_FLAG;
        c_can_frame.can_dlc = dlc as u8;

        Some(Self(c_can_frame))
    }

    fn is_extended(&self) -> bool {
        self.0.can_id & libc::CAN_EFF_FLAG != 0
    }

    fn is_remote_frame(&self) -> bool {
        self.0.can_id & libc::CAN_RTR_FLAG != 0
    }

    fn id(&self) -> embedded_hal::can::Id {
        match self.is_extended() {
            true => {
                let extended_id = can::ExtendedId::new(self.0.can_id & libc::CAN_EFF_MASK).unwrap();
                can::Id::Extended(extended_id)
            }
            false => {
                let standard_id =
                    can::StandardId::new((self.0.can_id & libc::CAN_SFF_MASK) as u16).unwrap();
                can::Id::Standard(standard_id)
            }
        }
    }

    fn dlc(&self) -> usize {
        self.0.can_dlc as usize
    }

    fn data(&self) -> &[u8] {
        &self.0.data
    }
}
