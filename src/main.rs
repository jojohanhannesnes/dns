mod byte_packet_buffer;
mod headers;
mod packet;
mod query_type;
mod questions;
mod records;
mod result_code;
mod types;

use byte_packet_buffer::BytePacketBuffer;
use packet::DnsPacket;
use std::{fs::File, io::Read};
use types::Result;
fn main() -> Result<()> {
    let mut f = File::open("response_packet.txt")?;
    let mut buffer = BytePacketBuffer::new();
    f.read_exact(&mut buffer.buf)?;

    let packet = DnsPacket::from_buffer(&mut buffer)?;
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
