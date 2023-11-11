use std::{
    path::Path,
    fs::File,
    thread::sleep,
    time::Duration
};
use cpal::{StreamError, Sample, traits::{DeviceTrait, HostTrait, StreamTrait}, StreamConfig, SampleRate};
use deears::dsp::Producer;

fn main() {
    // file loading
    //let path: File = File::open(Path::new("./song.ogg")).expect("provide a song!");

    // ogg parsing
    //let mut ogg_stream = OggStreamReader::new(path).expect("make it readable!");
    //let sample_rate = ogg_stream.ident_hdr.audio_sample_rate;
    // let's load it all into memory (that's a good idea!)
    // let mut music: Vec<i16> = Vec::new();
    // while let Some(channels) = ogg_stream.read_dec_packet_itl().unwrap() {
    //     music.extend(channels);
    // }
    let u = deears::dsps::mem_producer::from_ogg_file(Path::new("./song.ogg")).unwrap();

    println!("Sample rate: {}", u.sample_rate());

    let device = cpal::default_host().default_output_device().expect("have an audio device");
    let config: StreamConfig = device.default_output_config().unwrap().config();

    let mut uu = deears::dsps::rate_adjuster::RateAdjuster{
        underlying: Box::new(u),
        target_rate: config.sample_rate.0 as usize
    };

    let mut counter = 0;

    println!("Sample rate for output: {:?}", config.sample_rate);
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = f32::from_sample(uu.amplitude(counter));
                counter += 1;
            }
        },
        |err: StreamError| eprintln!("oh no, {err}!"),
        None
    ).unwrap();
    stream.play().unwrap();

    sleep(Duration::from_secs(277));
}
