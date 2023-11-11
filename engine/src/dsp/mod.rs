pub mod mem_producer;
pub mod rate_adjuster;

#[derive(Copy, Clone, Debug)]
pub struct Attributes {
    pub sample_rate: usize,
    pub channels: usize
}

pub trait Producer: Send{

    fn amplitude(&mut self, sample: usize, channel: u8) -> f64;

    fn attributes(&self) -> Attributes;
}

pub type AnyProducer = Box<dyn Producer>;