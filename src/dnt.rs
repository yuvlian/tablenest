use byteorder::{LittleEndian, ReadBytesExt};
use read_from::ReadFrom;
use std::io::{self, Read, Write};

pub trait WriteCell {
    /// What error can happen when trying to write?
    type Error;

    /// Attempts to write `self` to the given output stream, returning the number
    /// of bytes written on success.
    fn write_to<W: Write>(&self, output: W) -> Result<(), Self::Error>;
}

macro_rules! impl_write_cell {
    ($type:ty) => {
        impl WriteCell for $type {
            type Error = io::Error;

            fn write_to<W: Write>(&self, mut output: W) -> Result<(), Self::Error> {
                write!(&mut output, "{}", self.0)
            }
        }
    };
    // Multiple types variant
    ( $( $type:ty ),+ ) => {
        $( impl_write_cell!($type); )+
    };
}

#[derive(Debug, Clone)]
pub struct UINT8(pub u8);

impl ReadFrom for UINT8 {
    type Error = io::Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        input.read_u8().map(|x| UINT8(x))
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

#[derive(Debug, Clone)]
pub struct LPNNTS(pub Vec<u8>);

impl ReadFrom for LPNNTS {
    type Error = io::Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        let len = UINT16::read_from(&mut input)?;
        let mut buffer = vec![0u8; len.0 as usize];
        input.read_exact(&mut buffer)?;
        Ok(LPNNTS(buffer))
    }
}

impl WriteCell for LPNNTS {
    type Error = io::Error;

    fn write_to<W: Write>(&self, mut output: W) -> Result<(), Self::Error> {
        output.write(&self.0)?;
        Ok(())
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

#[derive(Debug, Clone)]
pub struct INT32(pub i32);

impl ReadFrom for INT32 {
    type Error = io::Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        input.read_i32::<LittleEndian>().map(|x| INT32(x))
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

#[derive(Debug, Clone)]
pub struct FLOAT64(pub f64);

impl ReadFrom for FLOAT64 {
    type Error = io::Error;

    fn read_from<R: Read>(mut input: R) -> Result<Self, Self::Error> {
        input.read_f64::<LittleEndian>().map(|x| FLOAT64(x))
    }
}

impl_write_cell!(UINT8, UINT16, UINT32, INT32, FLOAT32, FLOAT64);

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
            data_type: UINT8::read_from(&mut input)?,
        })
    }
}
