use crate::dsp::{AnyProducer, Attributes, Producer};

pub struct Sum{
    xs: Vec<AnyProducer>
}

impl Sum{
    pub fn new(xs: Vec<AnyProducer>) -> Self{
        // assuming they have the right sample rates...
        Self{ xs }
    }
}

impl Producer for Sum{
    fn amplitude(&mut self, sample: u64, channel: u8) -> f64{
        self.xs.iter_mut().map(|x| x.amplitude(sample, channel)).sum::<f64>() / (self.xs.len() as f64)
    }

    fn attributes(&self) -> Attributes{
        self.xs[0].attributes()
    }
}