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
        //let (secs, sms) = (time / self.rate, time % self.rate);
        /*std::f64::consts::PI;*/

        ((time as f64) * std::f64::consts::PI * 2f64 * (self.frequency / self.rate as f64)).sin()
    }

    fn attributes(&self) -> Attributes{
        Attributes{
            channels: 2,
            sample_rate: self.rate
        }
    }
}