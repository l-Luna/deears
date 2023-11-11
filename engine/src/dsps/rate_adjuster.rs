use crate::dsp::Producer;

pub struct RateAdjuster{
    pub underlying: Box<dyn Producer + Send>,
    pub target_rate: usize
}

impl Producer for RateAdjuster {

    fn amplitude(&mut self, sample: usize) -> f64 {
        let ur = self.underlying.sample_rate() as usize;
        let (secs, off) = (sample / self.target_rate, sample % self.target_rate);
        // pretty much just for when the rates are equal
        if off == 0 {
            return self.underlying.amplitude(secs * ur);
        } else {
            let base = secs * ur;
            let frac = (off as f64 / self.sample_rate() as f64) * ur as f64;
            let (hiS, loS, d) = (base + frac.ceil() as usize, base + frac.floor() as usize, frac % 1f64);
            return d * self.underlying.amplitude(hiS) + (1f64 - d) * self.underlying.amplitude(loS);
        }
    }

    fn sample_rate(&self) -> usize {
        self.target_rate
    }
}