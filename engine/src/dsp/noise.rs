use rand::random;
use crate::dsp::{Attributes, Producer};

pub struct Noise;

impl Producer for Noise {

    fn amplitude(&mut self, _: u64, _: u8) -> f64 {
        random()
    }

    // this like actually doesn't matter
    // or more generally, *these are negotioble*
    fn attributes(&self) -> Attributes {
        Attributes{
            channels: 2,
            sample_rate: 1000
        }
    }
}