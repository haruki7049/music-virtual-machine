use clap::Parser;
use std::f32::consts::PI;
use std::i16;
use std::path::PathBuf;
use hound::WavSpec;
use music_virtual_machine::mvm::{self, MusicData};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();
    //let music_data: MusicData = mvm::parse(&args.path)?;

    //let spec = WavSpec {
    //    channels: music_data.channels,
    //    sample_rate: music_data.sample_rate,
    //    bits_per_sample: music_data.bits_per_sample,
    //    sample_format: music_data.sample_format.into(),
    //};
    //let mut writer = hound::WavWriter::create(args.path, spec)?;
    //for bit in music_data.bits {
    //    writer.write_sample(bit as i16)?;
    //}

    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    eprintln!("channels = 1");
    eprintln!("sample_rate = 44100");
    eprintln!("bits_per_sample = 16");
    eprintln!("sample_format = \"Int\"");
    eprintln!("");
    eprintln!("bits = [");

    for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        eprintln!("    {},", sample * amplitude);
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }

    writer.finalize()?;

    eprintln!("]");

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(version, author, about)]
struct CLIArgs {
    /// The file path to TOML file contains Music VM data
    path: PathBuf,
}
