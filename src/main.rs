use std::{
    path::Path,
    fs::File,
    thread::sleep,
    time::Duration
};
use cpal::{
    StreamError,
    Sample,
    traits::{DeviceTrait, HostTrait, StreamTrait}
};
use lewton::inside_ogg::OggStreamReader;
use rand::Rng;

fn main() {
    // file loading
    let path: File = File::open(Path::new("./song.ogg")).expect("provide a song!");

    // ogg parsing
    let mut ogg_stream = OggStreamReader::new(path).expect("make it readable!");
    let sample_rate = ogg_stream.ident_hdr.audio_sample_rate;
    // let's load it all into memory (that's a good idea!)
    let mut music: Vec<i16> = Vec::new();
    while let Some(channels) = ogg_stream.read_dec_packet_itl().unwrap() {
        music.extend(channels);
    }

    println!("Sample rate: {}", sample_rate);

    let mut counter = 0;

    let device = cpal::default_host().default_output_device().expect("have an audio device");
    let config = device.default_output_config().unwrap().into();
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
            println!("{}", data.len());
            for sample in data.iter_mut() {
                *sample = f32::from_sample(music[counter]);//rand::thread_rng().gen_range(0f32..1f32)
                counter += 1;
            }
        },
        |err: StreamError| eprintln!("oh no, {err}!"),
        None
    ).unwrap();
    stream.play().unwrap();

    sleep(Duration::from_secs(277));
}
