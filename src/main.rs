use std::{
    path::Path,
    thread::sleep,
    time::Duration
};
use cpal::{StreamError, Sample, traits::{DeviceTrait, HostTrait, StreamTrait}, StreamConfig};
use deears::dsp::Producer;

fn main() {
    let u = deears::dsp::mem_producer::from_ogg_file(Path::new("./song.ogg")).unwrap();

    println!("Sample rate: {}", u.attributes().sample_rate);

    let device = cpal::default_host().default_output_device().expect("have an audio device");
    let config: StreamConfig = device.default_output_config().unwrap().config();

    let mut uu = deears::dsp::rate_adjuster::RateAdjuster{
        underlying: Box::new(u),
        target_rate: config.sample_rate.0 as usize
    };

    let mut counter = 0;

    println!("Sample rate for output: {:?}", config.sample_rate);
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = f32::from_sample(uu.amplitude(counter / 2, (counter % 2) as u8));
                counter += 1;
            }
        },
        |err: StreamError| eprintln!("oh no, {err}!"),
        None
    ).unwrap();
    stream.play().unwrap();

    sleep(Duration::from_secs(277));
}
