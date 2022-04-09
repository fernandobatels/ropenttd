//! Chunk reader and related API
//!
//! This is an API to access, and read, the data chunks
//! of OpenTTD.

use subslice::bmh;
use bytes::{Bytes, Buf};
use std::str;
use std::sync::Mutex;

use crate::bitmath::has_bit;
use crate::error::Error;

/// Chunk reader API
pub struct ChunkReader {
    raw: Mutex<Bytes>,
    /// Size of current chunk slice
    pub gamma: usize
}

impl ChunkReader {

    /// Find the chunk
    pub fn find(buffer: &Vec<u8>, chunk_id: &str) -> Result<ChunkReader, Error> {

        let pos_init = match bmh::find(buffer, chunk_id.as_bytes()) {
            Some(pos) => Ok(pos),
            None => Err(Error::ChunkNotFound(chunk_id.to_string()))
        }?;

        let (_,chunk) = buffer.split_at(pos_init);

        // We only support the CH_ARRAY and CH_SPARSE_ARRAY chunk types
        if chunk[4] != 1 && chunk[4] != 2 {
            return Err(Error::ChunkNotSupported(chunk_id.to_string()));
        }

        let mut chunk = Bytes::copy_from_slice(&chunk);
        chunk.advance(5); // chunk id + chunk type

        let gamma = read_gamma(&mut chunk)? as usize;

        Ok(ChunkReader {
            raw: Mutex::new(chunk),
            gamma: gamma
        })
    }

    /// Advance the cursor to the next chunk slice/value
    ///
    /// More about the chunk: https://github.com/OpenTTD/OpenTTD/blob/master/docs/savegame_format.md#chunks
    pub fn advance_slice(mut self) -> Result<Option<ChunkReader>, Error> {
        let mut raw = self.raw.get_mut()?;
        raw.advance(self.gamma - 1);

        self.gamma = read_gamma(&mut raw)? as usize;

        if self.gamma == 0 {
            return Ok(None);
        }

        Ok(Some(self))
    }

    /// Advance de cursor to the next value
    /// without return anything
    pub fn advance<T>(&mut self) -> Result<(), Error>
        where T: ChunkDataReader<T>
    {
        match self.fetch::<T>() {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    /// Fetch the next value from chunk
    pub fn fetch<T>(&mut self) -> Result<T, Error>
        where T: ChunkDataReader<T>
    {
        let raw = self.raw.get_mut()?;

        T::fetch(raw)
    }
}

/// Type of data supported on chunk
pub trait ChunkDataReader<T> {
    fn fetch(raw: &mut Bytes) -> Result<T, Error>;
}

/// SLE_FILE_I8
impl ChunkDataReader<i8> for i8 {
    fn fetch(raw: &mut Bytes) -> Result<i8, Error> {
        Ok(raw.get_i8())
    }
}

/// SLE_FILE_U8
impl ChunkDataReader<u8> for u8 {
    fn fetch(raw: &mut Bytes) -> Result<u8, Error> {
        Ok(raw.get_u8())
    }
}

/// SLE_FILE_I16
impl ChunkDataReader<i16> for i16 {
    fn fetch(raw: &mut Bytes) -> Result<i16, Error> {
        Ok(raw.get_i16())
    }
}

/// SLE_FILE_U16
impl ChunkDataReader<u16> for u16 {
    fn fetch(raw: &mut Bytes) -> Result<u16, Error> {
        Ok(raw.get_u16())
    }
}

/// SLE_FILE_I32
impl ChunkDataReader<i32> for i32 {
    fn fetch(raw: &mut Bytes) -> Result<i32, Error> {
        Ok(raw.get_i32())
    }
}

/// SLE_FILE_U32
impl ChunkDataReader<u32> for u32 {
    fn fetch(raw: &mut Bytes) -> Result<u32, Error> {
        Ok(raw.get_u32())
    }
}

/// SLE_VAR_I64
impl ChunkDataReader<i64> for i64 {
    fn fetch(raw: &mut Bytes) -> Result<i64, Error> {
        Ok(raw.get_i64())
    }
}

impl ChunkDataReader<String> for String {
    fn fetch(raw: &mut Bytes) -> Result<String, Error> {

        let len = raw.get_u8() as usize;
        let strb = raw.copy_to_bytes(len);

        let stru = str::from_utf8(&strb)?;

        Ok(stru.to_string())
    }
}

/// Returns the gamma value
///
/// More about gamma: https://github.com/OpenTTD/OpenTTD/blob/master/docs/savegame_format.md#gamma-value
pub(crate) fn read_gamma(raw: &mut Bytes) -> Result<u32, Error> {
    let mut r = raw.get_u8() as u32;
    if has_bit(r, 7) {
        r &= !0x80;
        if has_bit(r, 6) {
            r &= !0x40;
            if has_bit(r, 5) {
                r &= !0x20;
                if has_bit(r, 4) {
                    r &= !0x10;
                    if has_bit(r, 3) {
                        return Err(Error::DataCorruption(format!("Unsupported gamma: {}", r)));
                    }
                    r = raw.get_u8() as u32;
                }
                r = (r << 8) | raw.get_u8() as u32;
            }
            r = (r << 8) | raw.get_u8() as u32;
        }
        r = (r << 8) | raw.get_u8() as u32;
    }

    Ok(r)
}

#[cfg(test)]
mod test {

    use bytes::Bytes;
    use crate::chunk_reader::ChunkReader;
    use crate::chunk_reader::read_gamma;

    #[test]
    fn find() -> Result<(), String> {

        let bytes = vec![0x50, 0x4c, 0x59, 0x52, 0x1, 0x91, 0x1f, 0x83, 0x2a];
        let mut chunk = ChunkReader::find(&bytes, "PLYR")
            .map_err(|e| e.to_string())?;
        assert_eq!(4383, chunk.gamma);
        assert_eq!(33578, chunk.fetch::<u16>().map_err(|e| e.to_string())?);

        Ok(())
    }

    #[test]
    fn advance_slice() -> Result<(), String> {

        let bytes = vec![0x56, 0x45, 0x48, 0x53, 0x02, 0x1c, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0xff, 0x00, 0x00, 0x2d, 0x2e, 0x00, 0x00, 0x00, 0x4b, 0x0e, 0x77, 0x02, 0x04, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x01, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x5f, 0x00, 0x00, 0x1c, 0x1e, 0x00, 0x00, 0x00, 0x5b, 0x0e, 0x7b, 0x06, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];

        let chunk = ChunkReader::find(&bytes, "VEHS")
            .map_err(|e| e.to_string())?;
        assert_eq!(28, chunk.gamma);

        let chunk2 = chunk.advance_slice()
            .map_err(|e| e.to_string())?;
        assert_eq!(true, chunk2.is_some());
        let chunk2 = chunk2.unwrap();
        assert_eq!(28, chunk2.gamma);

        let chunk3 = chunk2.advance_slice()
            .map_err(|e| e.to_string())?;
        assert_eq!(false, chunk3.is_some());

        Ok(())
    }

    #[test]
    fn gamma() -> Result<(), String> {

        // PLYR example chunk
        let mut buffer = Bytes::copy_from_slice(&[0x91, 0x1f, 0x83, 0x2a]);
        assert_eq!(4383, read_gamma(&mut buffer).map_err(|e| e.to_string())?);

        // VEHS example chunk
        let mut buffer = Bytes::copy_from_slice(&[0x1c, 0x00, 0x04, 0x00]);
        assert_eq!(28, read_gamma(&mut buffer).map_err(|e| e.to_string())?);

        return Ok(());
    }
}
