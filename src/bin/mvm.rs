use clap::Parser;
use std::path::PathBuf;
use std::path::Path;
use std::ffi::OsStr;
use chrono::Local;
use chrono::DateTime;
use directories::ProjectDirs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();
    let output_path: PathBuf = if args.output.is_none() {
        output_path(&args.path)?
    } else {
        args.output.unwrap()
    };

    let music_data: music_virtual_machine::MusicData = music_virtual_machine::parse(&args.path)?;

    let spec = hound::WavSpec {
        channels: music_data.channels,
        sample_rate: music_data.sample_rate,
        bits_per_sample: music_data.bits_per_sample,
        sample_format: music_data.sample_format.into(),
    };

    let mut writer = hound::WavWriter::create(output_path, spec)?;

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
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn output_path(mvm_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let project_dirs: ProjectDirs = ProjectDirs::from("dev", "haruki7049", "mvm").ok_or("Failed to get project dir for dev.haruki7049.mvm")?;
    let cache_dir: &Path = project_dirs.cache_dir();
    let date: DateTime<Local> = Local::now();
    let toml_path: &OsStr = mvm_path.file_stem().ok_or("Failed to get filename of Music VM toml file")?;
    let result: PathBuf = format!("{}/{}-{}.wav", cache_dir.display(), toml_path.display(), date.timestamp_millis()).into();

    std::fs::create_dir_all(cache_dir)?;

    Ok(result)
}
