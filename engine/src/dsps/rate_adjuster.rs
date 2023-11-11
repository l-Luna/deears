use crate::dsp::Producer;

pub struct RateAdjuster{
    pub underlying: Box<dyn Producer + Send>,
    pub target_rate: u32
}

impl Producer for RateAdjuster {

    fn amplitude(&mut self, sample: usize) -> f64 {
        let mult: f64 = (self.underlying.sample_rate() as f64) / (self.target_rate as f64);
        return self.underlying.amplitude((mult * (sample as f64)) as usize)
    }

    fn sample_rate(&self) -> u32 {
        self.target_rate
    }
}