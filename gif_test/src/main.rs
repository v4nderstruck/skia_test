use image::{codecs::gif::GifDecoder, AnimationDecoder, Frame};
use indicatif::ProgressIterator;
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use std::{env, fs::File, mem::size_of_val};

// struct FrameBuffer(Frame);

// impl serde::ser::Serialize for FrameBuffer {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer {
//         let mut s = serializer.serialize_struct("FrameBuffer", 4)?;
//         s.serialize_field("top", &self.0.top())?;
//         s.serialize_field("delay", &self.0.delay().numer_denom_ms())?;
//         s.serialize_field("left", &self.0.left())?;
//         s.serialize_field("buffer", &self.0.buffer().as_raw())?;
//         s.end()
//     }
// }

// impl serde::de::Deserialize for FrameBuffer {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de> {
//     }
// }

fn load_image(file: &str) {
    println!("Loading image {}", file);
    let mut f = File::open(file).unwrap();
    let gif = GifDecoder::new(f).unwrap();
    let mut frames = gif.into_frames();
    // let frames = frames.collect_frames().unwrap();

    while let Some(Ok(f)) = frames.next() {
        println!("Frame Delay: {:?}", f.delay());
        let flat_sample = f.into_buffer();
        let flat_sample = flat_sample.into_flat_samples();
    }
    // println!("Total Frames: {:?}", frames.len());
    // println!("Total size: {:?}", size_of_val(&frames));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(file) => load_image(file),
        None => println!("No file to load"),
    }
}
