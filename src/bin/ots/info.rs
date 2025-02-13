use std::fs::File;
use std::fmt;
use std::io::{self, BufReader};
use std::num::NonZero;
use std::path::PathBuf;
use std::thread;

use thiserror::Error;
use bitcoin_hashes;

use opentimestamps::attestation::Attestation;
use opentimestamps::timestamp::detached::DetachedTimestampFile;
use opentimestamps::timestamp::Step;
use opentimestamps::op::{Op, HashOp};
use opentimestamps::hex::Hexed;

pub fn info_command(
    filename: PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let mut f = File::open(&filename)?;
    let ts = DetachedTimestampFile::deserialize(&mut f)?;

    println!("File {} hash: {:x}", ts.digest().to_hash_op(), Hexed(ts.digest()));
    println!("Timestamp:");

    let mut padding = 0;
    for step in ts.steps() {
        for _ in 0 .. padding {
            print!("    ");
        };

        match step {
            Step::Attestation(attestation) => {
                print!("verify ");
                match attestation {
                    Attestation::Bitcoin { block_height } => {
                        println!("BitcoinBlockHeaderAttestation({})", block_height);
                    },
                    Attestation::Pending(uri) => {
                        println!("Pending('{}')", uri);
                    },
                    Attestation::Unknown { tag, payload } => {
                        println!("UnknownAttestation({:x}, {:x})", Hexed(tag), Hexed(payload));
                    },
                }
                padding -= 1;
            },
            Step::Op(Op::HashOp(op)) => {
                println!("{}", op);
            },
            Step::Op(Op::Append(right)) => {
                println!("append {:x}", Hexed(right));
            },
            Step::Op(Op::Prepend(left)) => {
                println!("prepend {:x}", Hexed(left));
            },
            Step::Op(Op::Hexlify) => {
                println!("hexlify");
            },
            Step::Fork => {
                padding += 1;
            },
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
}
