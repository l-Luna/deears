use super::{Producer, AnyProducer, Attributes};

pub struct LowpassFilter {
    underlying: AnyProducer,
    ratio: f64,
    v0: f64
}

impl LowpassFilter {

    pub fn new(underlying: impl Producer + 'static, ratio: impl Into<f64>) -> Self{
        LowpassFilter {
            underlying: Box::new(underlying),
            ratio: ratio.into(),
            v0: 0.0
        }
    }
}

impl Producer for LowpassFilter {

    fn amplitude(&mut self, sample: u64, channel: u8) -> f64 {
        self.v0 = self.underlying.amplitude(sample, channel) * self.ratio + self.v0 * (1.0 - self.ratio);
        self.v0
    }

    fn attributes(&self) -> Attributes {
        self.underlying.attributes()
    }
}