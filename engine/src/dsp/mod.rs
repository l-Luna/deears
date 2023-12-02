pub mod mem_producer;
pub mod rate_adjuster;
pub mod noise;
pub mod lowpass_filter;
pub mod sine;
mod buffered;

#[derive(Copy, Clone, Debug)]
pub struct Attributes {
    pub sample_rate: u64,
    pub channels: u64
}

pub trait Producer: Send{

    fn amplitude(&mut self, sample: u64, channel: u8) -> f64;

    fn attributes(&self) -> Attributes;
}

pub type AnyProducer = Box<dyn Producer>;