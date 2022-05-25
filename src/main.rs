use std::env;
use std::path::PathBuf;

use snpsubtract::{load_snps, subtract_snps};

use anyhow::Result;

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();

    let vcfs_dir = PathBuf::from(args.get(1).unwrap());
    let snps_fp = PathBuf::from(args.get(2).unwrap());
    let vcfs_out_dir = PathBuf::from(args.get(3).unwrap());

    let snps = load_snps(&snps_fp)?;

    subtract_snps(&vcfs_dir, &vcfs_out_dir, &snps);

    Ok(())
}
