use argh::FromArgs;
use async_std::{fs::File, io::ReadExt};

use color_eyre::eyre;
use futures::io::AsyncRead;
use sha3::Digest;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::path::{Path, PathBuf};

/// Prints the SHA-256 hash of a file.
#[derive(FromArgs)]
struct Args {
    /// the file whose contents to hash and print
    #[argh(positional)]
    files: Vec<PathBuf>,
}

struct TracingReader<R> where R: AsyncRead {
    inner: R,
}

impl<R> AsyncRead for TracingReader<R> where R:AsyncRead {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        todo!();
    }
}

#[async_std::main]
async fn main() -> Result<(), eyre::Error> {
    color_eyre::install().unwrap();
    let args: Args = argh::from_env();

    let mut handles = Vec::new();

    for file in &args.files {
        let file = file.clone();
        let handle = async_std::task::spawn(async move {
            let res = hash_file(&file).await;
            if let Err(e) = res {
                println!("WHile hashing {}: {}", file.display(), e);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await;
    }

    Ok(())
}

async fn hash_file(path: &Path) -> Result<(), eyre::Error> {
    let mut file = File::open(path).await?;
    let mut hasher = sha3::Sha3_256::new();

    let mut buf = vec![0u8; 256 * 1024];
    loop {
        // check out the current thread id
        println!("{} => {:?}", path.display(), std::thread::current().id());
        let n = file.read(&mut buf[..]).await?;
        match n {
            0 => break,
            n => hasher.update(&buf[..n]),
        }
    }

    let hash = hasher.finalize();
    print!("{} ", path.display());
    for x in hash {
        print!("{:02x}", x);
    }
    println!();

    Ok(())
}

