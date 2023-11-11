use super::{Producer, AnyProducer, Attributes};

pub struct RateAdjuster{
    pub underlying: AnyProducer,
    pub target_rate: u64
}

impl RateAdjuster {

    pub fn new(underlying: impl Producer + 'static, target_rate: impl Into<u64>) -> Self{
        RateAdjuster{
            underlying: Box::new(underlying),
            target_rate: target_rate.into()
        }
    }
}

impl Producer for RateAdjuster {

    fn amplitude(&mut self, sample: u64, channel: u8) -> f64 {
        let ur = self.underlying.attributes().sample_rate;
        let (secs, off) = (sample / self.target_rate, sample % self.target_rate);
        // pretty much just for when the rates are equal
        if off == 0 {
            return self.underlying.amplitude(secs * ur, channel);
        } else {
            let base = secs * ur;
            let frac = (off as f64 / self.target_rate as f64) * ur as f64;
            let (hi_s, lo_s, d) = (base + frac.ceil() as u64, base + frac.floor() as u64, frac % 1f64);
            return d * self.underlying.amplitude(hi_s, channel) + (1f64 - d) * self.underlying.amplitude(lo_s, channel);
        }
    }

    fn attributes(&self) -> Attributes {
        return Attributes{sample_rate: self.target_rate, ..self.underlying.attributes()};
    }
}