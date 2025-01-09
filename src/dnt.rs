use std::fmt::{write, Display, Formatter};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, ErrorKind, Read};
use read_from::ReadFrom;

#[derive(Debug, Clone)]
pub struct UINT8(pub u8);

impl ReadFrom for UINT8 {
    type Error = io::Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        input.read_u8().map(|x| UINT8(x))
    }
}

impl Display for UINT8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct UINT16(pub u16);

impl ReadFrom for UINT16 {
    type Error = io::Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        input.read_u16::<LittleEndian>().map(|x| UINT16(x))
    }
}

impl Display for UINT16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct LPNNTS(pub String);

impl ReadFrom for LPNNTS {
    type Error = io::Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        let len = UINT16::read_from(&mut input)?;
        let mut buffer = vec![0u8; len.0 as usize];
        input.read_exact(&mut buffer)?;
        match String::from_utf8(buffer) {
            Ok(str) => Ok(LPNNTS(str)),
            Err(e) => Err(io::Error::new(ErrorKind::Other, e)),
        }
    }
}

impl Display for LPNNTS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct UINT32(pub u32);


impl ReadFrom for UINT32 {
    type Error = io::Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        input.read_u32::<LittleEndian>().map(|x| UINT32(x))
    }
}


impl Display for UINT32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct INT32(pub i32);

impl ReadFrom for INT32 {
    type Error = io::Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        input.read_i32::<LittleEndian>().map(|x| INT32(x))
    }
}

impl Display for INT32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct FLOAT32(pub f32);

impl ReadFrom for FLOAT32 {
    type Error = io::Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        input.read_f32::<LittleEndian>().map(|x| FLOAT32(x))
    }
}

impl Display for FLOAT32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct FLOAT64(pub f64);

impl ReadFrom for FLOAT64 {
    type Error = io::Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        input.read_f64::<LittleEndian>().map(|x| FLOAT64(x))
    }
}

impl Display for FLOAT64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub magic_number: UINT32,
    pub column_count: UINT16,
    pub row_count: UINT32,
}

impl ReadFrom for Header {
    type Error = io::Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        Ok(Header {
            magic_number: UINT32::read_from(&mut input)?,
            column_count: UINT16::read_from(&mut input)?,
            row_count: UINT32::read_from(&mut input)?,
        })
    }

}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: LPNNTS,
    pub data_type: UINT8,
}

impl ReadFrom for Column {
    type Error = io::Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        Ok(Column {
            name: LPNNTS::read_from(&mut input)?,
            data_type: UINT8::read_from(&mut input)?
        })
    }

}
