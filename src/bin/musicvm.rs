use clap::Parser;
use std::path::PathBuf;
use music_virtual_machine::mvm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();
    let music_data: mvm::MusicData = mvm::parse(&args.path)?;

    let spec = hound::WavSpec {
        channels: music_data.channels,
        sample_rate: music_data.sample_rate,
        bits_per_sample: music_data.bits_per_sample,
        sample_format: music_data.sample_format.into(),
    };

    let mut writer = hound::WavWriter::create(args.output, spec)?;

    for bit in music_data.bits {
        writer.write_sample(bit as i16)?;
    }

    writer.finalize()?;

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(version, author, about)]
struct CLIArgs {
    /// The file path to TOML file contains Music VM data
    path: PathBuf,

    /// Output wave file path
    output: PathBuf,
}
