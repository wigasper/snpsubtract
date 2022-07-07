use std::env;
use std::fs::create_dir;
use std::path::PathBuf;

use snpsubtract::{load_snps, subtract_snps};

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned()) {
        println!(
            "Usage:\n
                 $ snpsubtract vcfs_in_directory snps_db.vcf vcfs_out_directory"
        )
    } else {
        let vcfs_dir = PathBuf::from(args.get(1).unwrap());
        let snps_fp = PathBuf::from(args.get(2).unwrap());
        let vcfs_out_dir = PathBuf::from(args.get(3).unwrap());

        if !vcfs_out_dir.exists() {
            create_dir(&vcfs_out_dir)?;
        } else if !vcfs_out_dir.is_dir() {
            panic!("{:?} is not a directory", vcfs_out_dir);
        }

        let snps = load_snps(&snps_fp)?;

        subtract_snps(&vcfs_dir, &vcfs_out_dir, &snps)?;
    }

    Ok(())
}
