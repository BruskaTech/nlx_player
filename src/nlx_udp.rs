use std::{fs::File, io, io::{BufReader, Read, Cursor, ErrorKind}, error::Error};
use binrw::{BinRead, BinWrite};

use super::nlx_csc::NlxCscRecord;

const EXTRAS_LENGTH: usize = 10;

#[derive(Debug, Clone, BinRead, BinWrite)]
#[brw(little)]
pub struct NlxUdpPacket {
    pub stx: i32,
    pub packet_id: i32,
    pub packet_size: i32,
    pub timestamp_high: u32,
    pub timestamp_low: u32,
    pub status: i32,
    pub parallel_input_port: u32,
    pub extras: [i32; EXTRAS_LENGTH],
    #[br(count = packet_size as usize - EXTRAS_LENGTH)]
    pub data: Vec<i32>,
    pub crc: i32,
}

impl NlxUdpPacket {
    pub fn checksum(&self) -> u32 {
        let mut sum: u32 = 0;
        sum ^= self.stx as u32;
        sum ^= self.packet_id as u32;
        sum ^= self.packet_size as u32;
        sum ^= self.timestamp_high as u32;
        sum ^= self.timestamp_low as u32;
        sum ^= self.status as u32;
        sum ^= self.parallel_input_port as u32;
        for val in &self.extras {
            sum ^= *val as u32;
        }
        for val in &self.data {
            sum ^= *val as u32;
        }
        sum ^= self.crc as u32;
        sum
    }
}

// One channel for each record
pub fn csc_to_packets(records: Vec<NlxCscRecord>, first_id: Option<i32>) -> Result<Vec<NlxUdpPacket>, Box<dyn Error>> {
    let mut packets = Vec::new();
    let mut packet_id = first_id.unwrap_or(0);
    
    let sample_size = records[0].number_of_valid_samples as usize;
    let sampling_period = 1000000 / records[0].sample_frequency as u64;

    for (i, record) in records.iter().enumerate() {
        if (record.number_of_valid_samples as usize) != sample_size {
            let error_msg = format!("Invalid number of samples in record. Record {i} had {} samples when it expected {sample_size}. All records must have the same number of samples.", 
                record.number_of_valid_samples as usize);
            return Err(Box::new(io::Error::new(ErrorKind::InvalidData, error_msg)));
        }
    }

    for i in 0..sample_size {
        let timestamp = records[0].timestamp + (i as u64 * sampling_period);

        let packet = NlxUdpPacket {
            stx: 2048,
            packet_id: packet_id,
            packet_size: 1044,
            timestamp_high: (timestamp >> 32) as u32,
            timestamp_low: (timestamp & 0xFFFFFFFF) as u32,
            status: 0,
            parallel_input_port: 0,
            extras: [0; 10],
            data: records.iter()
                .map(|x| x.samples[i] as i32)
                .collect(),
            crc: 0,
        };
        packets.push(packet);
        packet_id += 1;
    }

    Ok(packets)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::{fs::File, io::{BufReader, Read, Cursor}};
    use crate::nlx_csc::*;
    
    #[test]
    fn test_nlx_udp() -> Result<(), Box<dyn std::error::Error>> {
        let mut bytes = [0; 16384];
        let file = File::open("/Users/bruskajp/Downloads/RawData.nrd").unwrap();
        let mut reader = BufReader::new(file);
        reader.read_exact(&mut bytes)?;

        // Read the bytes from the file into the struct
        let mut bytes = [0; 1096];
        reader.read_exact(&mut bytes)?;
        let mut cursor = Cursor::new(bytes);
        let packet = NlxUdpPacket::read(&mut cursor)?;
        let data1: Vec<u8> = bytes.iter().map(|x| *x as u8).collect();

        // Convert the struct back into bytes
        let mut writer = Cursor::new(Vec::new());
        packet.write(&mut writer)?;
        let data2 = writer.into_inner();

        assert_eq!(data1, data2);

        Ok(())
    }

    #[test]
    fn test_csc_to_packets() -> Result<(), Box<dyn Error>> {
        let path = "/Users/bruskajp/Downloads/csc_data/".to_owned();
        let (_, iterator1) = NlxCscFileIterator::new(path.clone()+"LA1.ncs", None)?;
        let (_, iterator2) = NlxCscFileIterator::new(path.clone()+"LA2.ncs", None)?;
        let iterator = iterator1.zip(iterator2);

        let mut i = 0;
        for (records1, records2) in iterator {
            let records1 = records1?;
            let records2 = records2?;

            println!("{} {}", records1.samples.len(), records2.samples.len());

            let packets = csc_to_packets(vec![records1.clone(), records2.clone()], Some(0))?.iter().map(|x| (x.data[0], x.data[1])).collect::<Vec<(i32, i32)>>();

            let data1 = records1.samples.iter().map(|x| *x as i32).collect::<Vec<i32>>();
            let data2 = records2.samples.iter().map(|x| *x as i32).collect::<Vec<i32>>();
            let data = data1.iter().copied().zip(data2.iter().copied()).collect::<Vec<(i32, i32)>>();

            //assert_eq!(packets.len(), 512);
            assert_eq!(data.len(), 512);
            println!("{} {}", data.len(), packets.len());

            assert_eq!(packets, data);

            println!("{i}");
            i += 1;
        }
        

        Ok(())
    }
}

