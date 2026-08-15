use super::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Packet {
    pub kind: PacketKind,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn to_vec(self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.data.len() + 1);
        vec.push(self.kind.to_code());
        vec.extend_from_slice(&self.data);
        vec
    }

    pub fn from_vec(mut vec: Vec<u8>) -> Option<Self> {
        let first = vec.get(0).copied()?;
        let kind = PacketKind::from_code(first)?;
        let data = vec.split_off(1);
        Some(Self { kind, data })
    }
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
