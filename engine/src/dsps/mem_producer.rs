use std::fs::File;
use std::path::Path;
use std::io::{Read, Seek};
use lewton::inside_ogg::OggStreamReader;
use lewton::samples::{InterleavedSamples, Sample};
use crate::dsp::Producer;

pub struct MemProducer {
    samples: Vec<f64>,
    sample_rate: usize
}

impl Producer for MemProducer {

    fn amplitude(&mut self, sample: usize) -> f64 {
        self.samples[sample]
    }

    fn sample_rate(&self) -> usize {
        self.sample_rate
    }
}

pub fn from_ogg_file(path: &Path) -> Option<impl Producer>{
    let Ok(file) = File::open(path) else { return None; };
    let Ok(mut ogg_reader) = lewton::inside_ogg::OggStreamReader::new(file) else { return None; };

    let mut music: Vec<f64> = Vec::new();
    while let Some(samples) = read_dec_packet_itl::<_, f32>(&mut ogg_reader).unwrap() {
        // wehh
        for sample in samples {
            music.push(sample as f64);
        }
    }

    return Some(MemProducer{samples: music, sample_rate: ogg_reader.ident_hdr.audio_sample_rate as usize});
}

pub fn read_dec_packet_itl<T: Read + Seek, S: Sample>(s: &mut OggStreamReader<T>) -> Result<Option<Vec<S>>, lewton::VorbisError> {
    let decoded_pck: InterleavedSamples<_> = match s.read_dec_packet_generic()? {
        Some(p) => p,
        None => return Ok(None),
    };
    return Ok(Some(decoded_pck.samples));
}