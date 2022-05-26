use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

use std::collections::HashMap;

use anyhow::Result;

type SNPsdb = HashMap<String, HashMap<usize, (String, Vec<String>)>>;

pub fn load_snps(fp: &PathBuf) -> Result<SNPsdb> {
    let file = File::open(fp)?;
    let reader = BufReader::new(file);

    let mut snps: SNPsdb = HashMap::new();

    for line in reader.lines() {
        let this_line = line.unwrap();

        if !this_line.starts_with("#") {
            let mut split = this_line.split("\t");
            let chr = split.next().unwrap().to_owned();
            let pos = split.next().unwrap().parse::<usize>()?;
            let ref_allele = split.nth(1).unwrap().to_owned();
            let alt: Vec<String> = split
                .next()
                .unwrap()
                .split(",")
                .map(|s| s.to_owned())
                .collect();

            let _ = snps.entry(chr.to_owned()).or_insert(HashMap::new());
            snps.get_mut(&chr).unwrap().insert(pos, (ref_allele, alt));
        }
    }

    Ok(snps)
}

pub fn process_one_vcf(in_fp: &PathBuf, out_fp: &PathBuf, snps: &SNPsdb) -> Result<()> {
    let mut out_buffer = File::create(out_fp).unwrap();

    let reader = BufReader::new(File::open(in_fp)?);

    println!("processing {:?}", out_fp);

    for line in reader.lines() {
        let this_line = line.unwrap();

        if this_line.starts_with("#") {
            out_buffer.write(format!("{}\n", this_line).as_bytes())?;
        } else {
            let mut split = this_line.split("\t");
            let chr = split
                .next()
                .unwrap()
                .strip_prefix("chr")
                .unwrap()
                .to_owned();
            let pos = split.next().unwrap().parse::<usize>()?;
            let ref_allele = split.nth(1).unwrap().to_owned();
            let alt = split.next().unwrap().to_owned();

            if !snps.contains_key(&chr) {
                out_buffer.write(format!("{}\n", this_line).as_bytes())?;
            } else if !snps.get(&chr).unwrap().contains_key(&pos) {
                out_buffer.write(format!("{}\n", this_line).as_bytes())?;
            } else {
                //check make sure ref matches actual ref
                let actual_ref = &snps.get(&chr).unwrap().get(&pos).unwrap().0;

                if actual_ref.len() == 1 && ref_allele.len() == 1 && actual_ref != &ref_allele {
                    println!(
                        "{chr}\t{pos}\t{actual_ref} != {ref_allele}, are you using the correct
                        reference genome?"
                    );
                }
                //

                if !snps.get(&chr).unwrap().get(&pos).unwrap().1.contains(&alt) {
                    out_buffer.write(format!("{}\n", this_line).as_bytes())?;
                }
            }
        }
    }

    Ok(())
}

pub fn subtract_snps(vcfs_in: &PathBuf, vcfs_out_dir: &PathBuf, snps: &SNPsdb) -> Result<()> {
    let vcf_fps = fs::read_dir(vcfs_in).unwrap();

    for vcf in vcf_fps {
        let vcf_in_fp = vcf.unwrap().path();
        let out_fp = vcfs_out_dir.join(vcf_in_fp.file_name().unwrap());
        process_one_vcf(&vcf_in_fp, &out_fp, snps)?;
    }

    Ok(())
}
