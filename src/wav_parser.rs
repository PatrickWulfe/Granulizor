extern crate hound;

use hound::WavReader;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Copy, Clone)]
pub struct StereoFrame (pub f32, pub f32);

impl StereoFrame {
    pub fn copy(&self) -> StereoFrame{
        StereoFrame(self.0, self.1)
    }

    pub fn get_left(&self) -> f32{
        return self.0;
    }

    pub fn get_right(&self) -> f32 {
        return self.1;
    }

    pub fn get_mono(&self) -> f32 {
        return (self.0 + self.1) / 2.0;
    }

    pub fn set_value(&mut self, channel: u8, value: f32) {
        if channel == 0 {
            self.0 = value;
        } else {
            self.1 = value;
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Hound(hound::Error),
    UnsupportedBitsPerSample(u16),
}

pub fn parse_wav(path: &str) -> Result<Vec<StereoFrame>, Error> {
    // Use hound to open wav file and determine file format
    let reader = WavReader::open(path).unwrap();
    let spec = reader.spec();

    println!("{:?} {:?}", spec.sample_format, spec.bits_per_sample);

    let mut samples = match spec.sample_format {
        hound::SampleFormat::Float => match spec.bits_per_sample {
            32 => match parse_wav_float(reader) {
                Ok(samples) => samples,
                _ => return Err(Error::UnsupportedBitsPerSample(1)),
            },
            n => return Err(Error::UnsupportedBitsPerSample(n)),
            _ => return Err(Error::UnsupportedBitsPerSample(2)),
        },
        hound::SampleFormat::Int => match spec.bits_per_sample {
            32 => match parse_wav_int(reader) {
                Ok(samples) => samples,
                _ => return Err(Error::UnsupportedBitsPerSample(3)),
            },
            16 => match parse_wav_int(reader) {
                Ok(samples) => samples,
                _ => return Err(Error::UnsupportedBitsPerSample(4)),
            },
            _ => return Err(Error::UnsupportedBitsPerSample(5)),
        },
    };

    let mut result = Vec::new();
    let mut temp: StereoFrame = StereoFrame(0.0, 0.0);
    for (i, sample) in samples.iter().enumerate() {
        if i % 2 == 0 {
            temp.0 = *sample;
        } else {
            temp.1 = *sample;
            result.push(temp);
        }
    }
    Ok(result)
}

pub fn parse_wav_float(mut reader: WavReader<BufReader<File>>) -> Result<Vec<f32>, Error> {
    let mut samples = Vec::new();
    for sample in reader.samples() {
        match sample {
            Ok(smpl) => samples.push(smpl),
            _ => return Err(Error::UnsupportedBitsPerSample(6)),
        }
    }
    Ok(samples)
}

pub fn parse_wav_int(mut reader: WavReader<BufReader<File>>) -> Result<Vec<f32>, Error> {
    let mut samples = Vec::new();
    for sample in reader.samples() {
        match sample {
            Ok(smpl) => samples.push(smpl),
            _ => return Err(Error::UnsupportedBitsPerSample(7)),
        }
    }
    Ok(samples)
}


#[cfg(test)]
mod tests {
    use wav_parser::parse_wav;

    #[test]
    fn test_parse_wav() {
        println!("{:?}", parse_wav("E:\\Devel\\Repositories\\School\\Granulizor\\assets\\pads.wav").unwrap().len());
    }
}