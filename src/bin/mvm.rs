use clap::Parser;
use std::path::PathBuf;
use std::path::Path;
use std::ffi::OsStr;
use chrono::Local;
use chrono::DateTime;
use directories::ProjectDirs;
use hound::SampleFormat;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    let args: CLIArgs = CLIArgs::parse();
    
    match args.output {
        None => {
            let output_path: PathBuf = output_path(&args.tomlfile)?;
            generate_wavefile(&args.tomlfile, &output_path)?;
        }
        Some(output_path) => {
            generate_wavefile(&args.tomlfile, &output_path)?;
        }
    }

    if args.play {
        //play(&output_path)
    }

    Ok(())
}

fn generate_wavefile(tomlfile: &PathBuf, output: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let music_data: music_virtual_machine::MusicData = music_virtual_machine::parse(tomlfile)?;

    let spec = hound::WavSpec {
        channels: music_data.channels,
        sample_rate: music_data.sample_rate,
        bits_per_sample: music_data.bits_per_sample,
        sample_format: music_data.sample_format.into(),
    };

    let mut writer = hound::WavWriter::create(output, spec)?;

    match &spec.sample_format {
        SampleFormat::Int => for bit in music_data.bits {
            writer.write_sample(bit as i32)?;
        }
        SampleFormat::Float => for bit in music_data.bits {
            writer.write_sample(bit as f32)?;
        }
    }

    writer.finalize()?;

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(version, author, about)]
struct CLIArgs {
    /// The file path to TOML file contains Music VM data
    tomlfile: PathBuf,

    /// Output wave file path
    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(short, long, default_value_t = false)]
    play: bool,
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
