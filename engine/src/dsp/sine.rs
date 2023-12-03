use crate::dsp::{Attributes, Producer};

pub struct Sine{
    rate: u64,
    frequency: f64
}

impl Sine{
    pub fn new(rate: u64, frequency: f64) -> Self{
        Self{ rate, frequency }
    }
}

impl Producer for Sine{

    fn amplitude(&mut self, time: u64, _: u8) -> f64{
        ((time as f64) * std::f64::consts::PI * 2.0 * (self.frequency / self.rate as f64)).sin()
    }

    fn attributes(&self) -> Attributes{
        Attributes{
            channels: 2,
            sample_rate: self.rate
        }
    }
}

// TODO: parameter format
pub struct DSine{
    rate: u64,
    frequency: fn(u64) -> f64,
    acc: f64
}

impl DSine{
    pub fn new(rate: u64, frequency: fn(u64) -> f64) -> Self{
        Self{ rate, frequency, acc: 0.0 }
    }
}

impl Producer for DSine{
    fn amplitude(&mut self, sample: u64, channel: u8) -> f64{
        self.acc += (self.frequency)(sample);
        (std::f64::consts::PI * 2.0 * (1.0 / self.rate as f64) * self.acc).sin()
    }

    fn attributes(&self) -> Attributes{
        Attributes{
            sample_rate: self.rate,
            channels: 2
        }
    }
}