use super::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Packet {
    pub kind: PacketKind,
    pub data: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PacketKind {
    Set,
    Get,
}

impl PacketKind {
    pub fn to_code(self) -> u8 {
        match self {
            Self::Set => 1,
            Self::Get => 2,
        }
    }

    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Set),
            2 => Some(Self::Get),
            _ => None,
        }
    }
}

impl fmt::Display for PacketKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Set => write!(f, "set"),
            Self::Get => write!(f, "get"),
        }
    }
}
