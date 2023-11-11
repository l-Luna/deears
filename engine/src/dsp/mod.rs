pub mod mem_producer;
pub mod rate_adjuster;

pub trait Producer: Send{

    fn amplitude(&mut self, sample: usize) -> f64;

    fn sample_rate(&self) -> usize;
}

pub type AnyProducer = Box<dyn Producer>;