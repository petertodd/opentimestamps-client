use std::fs::File;
use std::fmt;
use std::io::{self, BufReader};
use std::num::NonZero;
use std::path::PathBuf;
use std::thread;

use thiserror::Error;
use bitcoin_hashes;

use opentimestamps::timestamp::TimestampBuilder;
use opentimestamps::tree::MerkleTreeBuilder;

fn hash_file(path: PathBuf) -> io::Result<[u8; 32]> {
    let mut f = BufReader::new(File::open(&path).unwrap());

    let mut engine = bitcoin_hashes::sha256::Hash::engine();

    io::copy(&mut f, &mut engine)?;
    Ok(bitcoin_hashes::sha256::Hash::from_engine(engine)
                                    .to_byte_array())
}

#[derive(Debug, Error)]
pub enum StampCommandError {
    #[error("filename {filename} err {err}")]
    FileIo {
        filename: PathBuf,
        err: io::Error,
    }
}

pub fn stamp_command<'a>(
    filenames: impl Iterator<Item = PathBuf>,
    jobs: NonZero<usize>,
) -> Result<(), StampCommandError>
{
    let mut filenames = filenames.fuse();
    let mut hashes: Vec<(PathBuf, [u8; 32])> = vec![];

    let mut threads: Vec<(PathBuf, _)> = vec![];
    loop {
        while threads.len() < jobs.get() {
            if let Some(filename) = filenames.next() {
                threads.push((filename.clone(), thread::spawn(move || hash_file(filename))));
            } else {
                break
            }
        }

        if threads.len() == 0 {
            break
        }

        // Wait for threads to complete
        thread::yield_now();
        for (filename, thread) in threads.extract_if(.., |x| x.1.is_finished()) {
            match thread.join().expect("thread failed") {
                Ok(hash) => {
                    hashes.push((filename, hash));
                },
                Err(err) => {
                    return Err(StampCommandError::FileIo { filename, err });
                }
            }
        }
    }

    let tree = MerkleTreeBuilder::with_nonces(
        hashes.iter().map(|(_name, hash)| {
            let hash = Vec::<u8>::from(hash);
            TimestampBuilder::new(hash.into())
        }
    ));

    dbg!(tree);

    Ok(())
}

#[cfg(test)]
mod test {
}
