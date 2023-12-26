use std::{error::Error};

pub fn from_ut8_unaligned(bytes: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut result = Vec::new();
    for byte in bytes {
        let char = char::from_u32(*byte as u32).ok_or("Invalid UTF-8")?;
        result.push(char);
    }
    
    Ok(result.into_iter().collect())
}