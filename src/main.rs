use std::io::Cursor;
use std::time::Duration;
use std::error::Error;
use std::net::UdpSocket;

use binrw::BinWrite;

mod nlx_csc;
use nlx_csc::*;

mod nlx_udp;
use nlx_udp::*;

mod utilities;
use utilities::*;

mod nlx_header;
mod as_result;

fn main() -> Result<(), Box<dyn Error>>{
    let path = "/Users/bruskajp/Downloads/csc_data/".to_owned();
    //let nlx_csc_file = NlxCscFile::open(path, None)?;
    let (_, iterator1) = NlxCscFileIterator::new(path.clone()+"LA1.ncs", None)?;
    let (_, iterator2) = NlxCscFileIterator::new(path.clone()+"LA2.ncs", None)?;

    let iterator = iterator1.zip(iterator2);

    let mut packets = Vec::new();

    for (records1, records2) in iterator {
        let mut p = csc_to_packets(vec![records1?, records2?], Some(0))?;
        packets.append(&mut p);

        break;
    }

    let socket = UdpSocket::bind("127.0.0.1:62877")?;
    periodic_for!(Duration::from_millis(10), packet in packets, {
        let mut writer = Cursor::new(Vec::new());
        packet.write(&mut writer)?;
        let data = writer.into_inner();
        socket.send_to(data.as_slice(),  "10.37.129.2:26090")?;
        println!("{:?}", data);
    });

    // let socket = UdpSocket::bind("127.0.0.1:62877")?;
    // for packet in packets {
    //     let mut writer = Cursor::new(Vec::new());
    //     packet.write(&mut writer)?;
    //     let data = writer.into_inner();
    //     println!("{:?}", data);
    //     socket.send_to(data.as_slice(),  "10.37.129.2:26090")?;


    //     sleep(Duration::from_millis(10));
    // }

    Ok(())
}


