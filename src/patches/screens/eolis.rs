#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EolisScreen {
    ExteriorGates = 0x00,
    InteriorEolis = 0x01,
    GuruExterior = 0x02,
    ShopsExterior = 0x03,
    RoadEast = 0x04,
    RoadWest = 0x05,
    TrainersExterior = 0x06,
    PalaceExterior = 0x07,
    ExitToTrunk = 0x08,
}

impl EolisScreen {
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x00 => Some(Self::ExteriorGates),
            0x01 => Some(Self::InteriorEolis),
            0x02 => Some(Self::GuruExterior),
            0x03 => Some(Self::ShopsExterior),
            0x04 => Some(Self::RoadEast),
            0x05 => Some(Self::RoadWest),
            0x06 => Some(Self::TrainersExterior),
            0x07 => Some(Self::PalaceExterior),
            0x08 => Some(Self::ExitToTrunk),
            _ => None,
        }
    }
}
