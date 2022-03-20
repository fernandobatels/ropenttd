//! Chunk reader and related API
//!
//! This is an API to access, and read, the data chunks
//! of OpenTTD.

use subslice::bmh;
use bytes::{Bytes, Buf};
use std::str;
use std::sync::Mutex;

use crate::error::Error;

/// Chunk reader API
pub struct ChunkReader {
    raw: Mutex<Bytes>
}

impl ChunkReader {

    /// Find the chunk
    pub fn find(buffer: &Vec<u8>, chunk_id: &str) -> Result<ChunkReader, Error> {

        let pos_init = match bmh::find(buffer, chunk_id.as_bytes()) {
            Some(pos) => Ok(pos),
            None => Err(Error::ChunkNotFound(chunk_id.to_string()))
        }?;

        let (_,chunk) = buffer.split_at(pos_init);

        // We only support the CH_ARRAY chunk type
        if chunk[4] != 1 {
            return Err(Error::ChunkNotSupported(chunk_id.to_string()));
        }

        let mut chunk = Bytes::copy_from_slice(&chunk);
        chunk.advance(5); // chunk id + chunk type

        chunk.advance(2); // Some bytes not understood yet

        Ok(ChunkReader {
            raw: Mutex::new(chunk)
        })
    }

    /// Advance de cursor to the next value
    /// without return anything
    #[allow(dead_code)]
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

impl ChunkDataReader<String> for String {
    fn fetch(raw: &mut Bytes) -> Result<String, Error> {

        let len = raw.get_u8() as usize;
        let strb = raw.copy_to_bytes(len);

        let stru = str::from_utf8(&strb)?;

        Ok(stru.to_string())
    }
}
