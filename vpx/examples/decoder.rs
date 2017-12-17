//! Example usage of the vpx-rs library to build a simple VP8 decoder.
//! This program takes as input the path to an IVF file that contains VP8 frames, and writes the
//! decoded frames to the specified output folder.

extern crate vpx;

use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use vpx::{Codec, Decoder};

struct Video {
    f: File,
    frames_num: u32,
    read_frames_num: u32,
}

impl Video {
    fn open(path: String) -> Video {
        let mut f = File::open(path).expect("Failed to open file");

        let mut sig = vec![0u8; 4];
        f.read(&mut sig).unwrap();

        if String::from_utf8(sig).unwrap() != "DKIF" {
            panic!("Invalid IVF input: wrong signature");
        }

        // Ignore unused IVF header fields (version, length, fourcc, width, height, framerate,
        // timescale => 20 bytes).
        f.seek(SeekFrom::Current(20)).unwrap();

        let mut frames = vec![0u8; 4];
        f.read(&mut frames).unwrap();

        let mut frames_num = 0 as u32;
        for i in 0..4 {
            frames_num |= (frames[4 - i - 1] as u32) << (4 - i - 1) * 8;
        }

        let mut unused = vec![0u8; 4];
        f.read(&mut unused).unwrap();

        Video {
            f: f,
            frames_num: frames_num,
            read_frames_num: 0,
        }
    }
}

// Iterator over the video frames.
impl ::std::iter::Iterator for Video {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.read_frames_num >= self.frames_num {
            return None;
        }

        let mut size_buf = vec![0u8; 4];
        self.f.read(&mut size_buf).unwrap();

        let mut size = 0 as u32;
        for i in 0..4 {
            size |= (size_buf[4 - i - 1] as u32) << (4 - i - 1) * 8;
        }

        // Skip the timestamp (unused).
        self.f.seek(SeekFrom::Current(8)).unwrap();

        let mut data = vec![0u8; size as usize];
        self.f.read(&mut data).unwrap();

        self.read_frames_num = self.read_frames_num + 1;
        Some(data)
    }
}

fn main() {
    let input = env::args().nth(1).expect("Please specify an input IVF file");
    let output = env::args().nth(2).expect("Please specify an output folder for the decoded frames");

    let video = Video::open(input);
    let mut decoder = Decoder::new(Codec::VP8).unwrap();

    for frame in video {
        for image in decoder.decode(frame).unwrap() {
            println!("Decoded image: {}x{}", image.width(), image.height());
        }
    }
}
