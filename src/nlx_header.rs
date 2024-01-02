use std::{collections::HashMap, error::Error};

use super::utilities::*;

pub const NLX_HEADER_SIZE: usize = 16384;

#[derive(Debug, PartialEq)]
pub struct NlxHeader {
    pub dict: HashMap<String, String>,
}

impl NlxHeader {
    pub fn new() -> NlxHeader {
        NlxHeader {
            dict: HashMap::new(),
        }
    }

    pub fn deserialize(data: &[u8; NLX_HEADER_SIZE]) -> Result<NlxHeader, Box<dyn Error>> {
        let mut hashmap = HashMap::new();

        let string = from_ut8_unaligned(data)?;

        for line in string.lines() {
            if !line.starts_with('-') {
                continue;
            }
            
            if let Some((key, value)) = line.split_once(' ') {
                hashmap.insert(key[1..].to_string(), value.to_string());
            }
        }
    
        Ok(NlxHeader {
            dict: hashmap,
        })
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_functions() {

        let nlx_csc_header = NlxHeader::new();
        assert_eq!(nlx_csc_header.dict, HashMap::new());
    }
}