use rand::random;
use crate::dsp::{Attributes, Producer};

pub struct Noise{
    rate: u64
}

impl Noise{
    pub fn new(rate: u64) -> Self{
        Self{ rate }
    }
}

impl Producer for Noise{

    fn amplitude(&mut self, _: u64, _: u8) -> f64{
        random()
    }

    fn attributes(&self) -> Attributes{
        Attributes{
            channels: 2,
            sample_rate: self.rate
        }
    }
}