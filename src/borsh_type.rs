use std::io::Error;
use borsh::{BorshDeserialize, BorshSerialize};  

use crate::CustomSerializer;

#[derive(Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BorshData;

impl<T: BorshSerialize + BorshDeserialize> CustomSerializer<T, Error> for BorshData {
    fn convert_to_bytes(&self, data: T) -> Result<Vec<u8>, Error> {
        Ok(borsh::to_vec(&data)?)
    }

    fn convert_from_bytes(&self, bytes: Vec<u8>) -> Result<T, Error> {
        Ok(T::try_from_slice(&bytes)?)
    }
}
