use crate::dsp::{AnyProducer, Attributes, Producer};

pub struct Buffered {
    underlying: AnyProducer,
    buffer: Vec<f64>,
    start_sample: u64,
}

impl Buffered {

    pub fn new(underlying: impl Producer + 'static, buffer_size: usize) -> Self{
        Buffered{
            underlying: Box::new(underlying),
            buffer: vec![0.0; buffer_size],
            start_sample: 0
        }
    }
}

impl Producer for Buffered {
    fn amplitude(&mut self, sample: u64, channel: u8) -> f64 {
        let offset = sample - self.start_sample;
        if offset < self.buffer.len() as u64 {
            return self.buffer[(offset * self.underlying.attributes().channels) as usize + channel as usize];
        }
        return 0.0;
    }

    fn attributes(&self) -> Attributes {
        self.underlying.attributes()
    }
}