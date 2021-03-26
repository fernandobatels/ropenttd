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

        Ok(ChunkReader {
            raw: Mutex::new(chunk)
        })
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

impl ChunkDataReader<u8> for u8 {
    fn fetch(raw: &mut Bytes) -> Result<u8, Error> {

        let tp = raw.get_u8() & 0xF;
        if tp != 1 { // SLE_FILE_I8
            return Err(Error::UnexpectedValueType(tp));
        }

        Ok(raw.get_u8())
    }
}

impl ChunkDataReader<u16> for u16 {
    fn fetch(raw: &mut Bytes) -> Result<u16, Error> {

        let tp = raw.get_u8() & 0xF;
        if tp != 3 { // SLE_FILE_U16
            return Err(Error::UnexpectedValueType(tp));
        }

        Ok(raw.get_u16())
    }
}

impl ChunkDataReader<String> for String {
    fn fetch(raw: &mut Bytes) -> Result<String, Error> {

        raw.advance(3); // Some bytes not understood yet

        let len = raw.get_u8() as usize;
        let strb = raw.copy_to_bytes(len);

        let stru = str::from_utf8(&strb)?;

        Ok(stru.to_string())
    }
}
