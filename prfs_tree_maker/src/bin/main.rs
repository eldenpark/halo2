use chrono::prelude::*;
use halo2_gadgets::{
    poseidon::{
        primitives::{self as poseidon, ConstantLength, P128Pow5T3 as OrchardNullifier, Spec},
        Hash,
    },
    utilities::UtilitiesInstructions,
};
use halo2_proofs::halo2curves::pasta::Fp;
use hyper::{body::HttpBody as _, Client, Uri};
use hyper::{Body, Method, Request, Response};
use hyper_tls::HttpsConnector;
use log::LevelFilter;
use prfs_tree_maker::{
    config::{END_BLOCK, GETH_ENDPOINT, START_BLOCK},
    leaves, TreeMakerError,
};
use std::fs::{File, OpenOptions};
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::main]
async fn main() -> Result<(), TreeMakerError> {
    println!("Tree maker starts");

    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let now = Utc::now();
    let log_files_path = project_root.join(format!("log_files/log"));

    println!("start time: {}", now);
    println!("log file path: {:?}", log_files_path);
    println!("geth endpoint: {}", GETH_ENDPOINT);

    println!(
        "start block no: {}, end block no: {}",
        START_BLOCK, END_BLOCK
    );

    if log_files_path.exists() == false {
        File::create(&log_files_path).unwrap();
    }

    simple_logging::log_to_file(log_files_path, LevelFilter::Error)?;

    leaves::make_leaves().await?;

    // grow::grow_tree().await?;
    // climb::climb_up().await?;

    Ok(())
}
