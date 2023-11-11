pub trait Producer{

    fn amplitude(&mut self, sample: usize) -> f64;

    fn sample_rate(&self) -> usize;
}