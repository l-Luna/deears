use super::{Producer, AnyProducer, Attributes};

pub struct RateAdjuster{
    pub underlying: AnyProducer,
    pub target_rate: usize
}

impl Producer for RateAdjuster {

    fn amplitude(&mut self, sample: usize, channel: u8) -> f64 {
        let ur = self.underlying.attributes().sample_rate as usize;
        let (secs, off) = (sample / self.target_rate, sample % self.target_rate);
        // pretty much just for when the rates are equal
        if off == 0 {
            return self.underlying.amplitude(secs * ur, channel);
        } else {
            let base = secs * ur;
            let frac = (off as f64 / self.target_rate as f64) * ur as f64;
            let (hi_s, lo_s, d) = (base + frac.ceil() as usize, base + frac.floor() as usize, frac % 1f64);
            return d * self.underlying.amplitude(hi_s, channel) + (1f64 - d) * self.underlying.amplitude(lo_s, channel);
        }
    }

    fn attributes(&self) -> Attributes {
        return Attributes{sample_rate: self.target_rate, ..self.underlying.attributes()};
    }
}