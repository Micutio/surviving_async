use argh::FromArgs;
use color_eyre::eyre;
use std::path::PathBuf;

/// Prints the SHA-256 hash of a file.
#[derive(FromArgs)]
struct Args {
    /// the file whose contents to hash and print
    #[argh(positional)]
    file: PathBuf,
}

fn main() -> Result<(), eyre::Error> {
    color_eyre::install().unwrap();

    let args: Args = argh::from_env();
    let metadata = std::fs::metadata(&args.file)?;

    println!("{} is {} bytes", args.file.display(), metadata.len());

    Ok(())
}
