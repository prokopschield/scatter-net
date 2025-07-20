use std::collections::VecDeque;

use ps_buffer::Buffer;

use crate::Packet;

#[derive(Debug)]
pub struct InteractionInnerWritable {
    pub buffer: Buffer,
    pub closed: bool,
    pub packets: VecDeque<Packet>,
}
