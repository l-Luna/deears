use std::{
    path::Path,
    thread::sleep,
    time::Duration
};
use cpal::{StreamError, Sample, traits::{DeviceTrait, HostTrait, StreamTrait}, StreamConfig};
use deears::dsp::lowpass_filter::LowpassFilter;
use deears::dsp::Producer;
use deears::dsp::rate_adjuster::RateAdjuster;
use deears::dsp::sine::DSine;

macro_rules! sum_of_sines {
    ($e:expr $(, $ex:expr)*) => {
        deears::dsp::sum::Sum::new(
            vec![ Box::new(deears::dsp::sine::Sine::new(48000, $e)) $( , Box::new(deears::dsp::sine::Sine::new(48000, $ex)) )* ]
        )
    };
}

fn main() {
    // deears::dsp::mem_producer::from_ogg_file(Path::new("./song.ogg")).unwrap();
    //sum_of_sines!(400.0,500.0,600.0,700.0);
    let u = DSine::new(48000, |s| ((s as f64) / 48000.0).sin() * 200.0 + 300.0);

    println!("Sample rate: {}", u.attributes().sample_rate);

    let device = cpal::default_host().default_output_device().expect("have an audio device");
    let config: StreamConfig = device.default_output_config().unwrap().config();

    let mut adjusted = /*LowpassFilter::new*/(RateAdjuster::new(u, config.sample_rate.0)/*, 0.6*/);

    let mut counter = 0;

    println!("Sample rate for output: {:?}", config.sample_rate);
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = f32::from_sample(adjusted.amplitude(counter / 2, (counter % 2) as u8));
                counter += 1;
            }
        },
        |err: StreamError| eprintln!("oh no, {err}!"),
        None
    ).unwrap();
    stream.play().unwrap();

    sleep(Duration::from_secs(277));
}
