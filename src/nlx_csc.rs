use std::{error::Error, path::Path, fs::File, io::{self, Read, BufReader}};


use super::nlx_header::*;
use super::nlx_udp::*;

const NLX_CSC_RECORD_SIZE: usize = 1044;

#[derive(Debug, PartialEq, Clone)]
pub struct NlxCscRecord {
    pub timestamp: u64,
    pub channel_number: u32,
    pub sample_frequency: u32,
    pub number_of_valid_samples: u32,
    pub samples: [i16; 512],
}

impl NlxCscRecord {
    pub fn new() -> NlxCscRecord {
        NlxCscRecord {
            timestamp: 0,
            channel_number: 0,
            sample_frequency: 0,
            number_of_valid_samples: 0,
            samples: [0; 512],
        }
    }

    pub fn deserialize(data: &[u8; NLX_CSC_RECORD_SIZE]) -> Result<NlxCscRecord, Box<dyn Error>> {
        let samples = data[20..]
            .chunks_exact(2)
            .map(|x| i16::from_le_bytes(x.try_into().unwrap()))
            .collect::<Vec<i16>>();

        Ok(NlxCscRecord {
            timestamp: u64::from_le_bytes(data[0..8].try_into()?),
            channel_number: u32::from_le_bytes(data[8..12].try_into()?),
            sample_frequency: u32::from_le_bytes(data[12..16].try_into()?),
            number_of_valid_samples: u32::from_le_bytes(data[16..20].try_into()?),
            samples: samples.try_into().unwrap(),
        })
    }

    pub fn serialize(&self) -> Result<[u8; NLX_CSC_RECORD_SIZE], Box<dyn Error>> {
        let mut data = [0; NLX_CSC_RECORD_SIZE];

        data[0..8].copy_from_slice(&self.timestamp.to_le_bytes());
        data[8..12].copy_from_slice(&self.channel_number.to_le_bytes());
        data[12..16].copy_from_slice(&self.sample_frequency.to_le_bytes());
        data[16..20].copy_from_slice(&self.number_of_valid_samples.to_le_bytes());
        data[20..].copy_from_slice(&self.samples.iter().map(|x| x.to_le_bytes()).flatten().collect::<Vec<u8>>());

        Ok(data)
    }
}

#[derive(Debug, PartialEq)]
pub struct NlxCscFile {
    pub header: NlxHeader,
    pub records: Vec<NlxCscRecord>,
}

impl NlxCscFile {
    pub fn open<P: AsRef<Path>>(path: P, num_records: Option<u64>) -> Result<NlxCscFile, Box<dyn Error>> {
        let file = File::open(&path)?;
        let mut reader = io::BufReader::new(file);
        
        // Get header
        let mut data = [0; NLX_HEADER_SIZE];
        reader.read_exact(&mut data)?;
        let header = NlxHeader::deserialize(&data)?;

        // Get all data records
        let mut records = Vec::new();
        let file_size = std::fs::metadata(path)?.len();
        let records_size = file_size - NLX_HEADER_SIZE as u64;

        // if file_size % NLX_CSC_RECORD_SIZE as u64 != 0 {
        //     return Err("File has extra bytes after record. The file may be corrupted.".into());
        // }

        let mut data = [0; NLX_CSC_RECORD_SIZE];
        for i in 0..(records_size / NLX_CSC_RECORD_SIZE as u64) {
            if Some(i) == num_records {
                break;
            }

            reader.read_exact(&mut data)?;
            let record = NlxCscRecord::deserialize(&data)?;
            records.push(record);
        }

        Ok(NlxCscFile {
            header: header,
            records: records,
        })
    }
}

pub struct NlxCscFileIterator {
    reader: BufReader<File>,
    num_records: u64,
    current_record: u64,
}

impl NlxCscFileIterator {
    pub fn new<P: AsRef<Path>>(path: P, num_records: Option<u64>) -> Result<(NlxHeader, NlxCscFileIterator), Box<dyn Error>> {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);

        let header = {
            let mut data = [0; NLX_HEADER_SIZE];
            reader.read_exact(&mut data)?;
            NlxHeader::deserialize(&data)?
        };

        let file_size = std::fs::metadata(path)?.len();
        let records_size = file_size - NLX_HEADER_SIZE as u64;
        let num_records = num_records.unwrap_or(records_size / NLX_CSC_RECORD_SIZE as u64);

        Ok((header,
            NlxCscFileIterator {
                reader: reader,
                num_records: num_records,
                current_record: 0,
            }
        ))
    }
}

impl Iterator for NlxCscFileIterator {
    type Item = Result<NlxCscRecord, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_record == self.num_records {
            return None;
        }

        let mut data = [0; NLX_CSC_RECORD_SIZE];
        match self.reader.read_exact(&mut data) {
            Ok(_) => {},
            Err(err) => return Some(Err(err.into())),
        }

        let record: Result<NlxCscRecord, Box<dyn Error>> = NlxCscRecord::deserialize(&data);
        self.current_record += 1;

        Some(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_functions() {
        let nlx_csc_record = NlxCscRecord::new();
        assert_eq!(nlx_csc_record.timestamp, 0);
        assert_eq!(nlx_csc_record.channel_number, 0);
        assert_eq!(nlx_csc_record.sample_frequency, 0);
        assert_eq!(nlx_csc_record.number_of_valid_samples, 0);
        assert_eq!(nlx_csc_record.samples, [0; 512]);
    }

    #[test]
    fn test_nlx_csc_file() -> Result<(), Box<dyn Error>> {
        let path = "/Users/bruskajp/Downloads/csc_data/LA2.ncs";

        let nlx_csc_file = NlxCscFile::open(path, None)?;

        let (header, iterator) = NlxCscFileIterator::new(path, None)?;

        assert_eq!(header, nlx_csc_file.header);
        for (i, record) in iterator.enumerate() {
            assert_eq!(record?, nlx_csc_file.records[i]);
        }

        Ok(())
    }
}