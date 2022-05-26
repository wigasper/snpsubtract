use snpsubtract::*;

use std::fs::remove_file;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[test]
fn test_full_run_0() {
    let snps = load_snps(&PathBuf::from("tests/test_data/snpdb.vcf")).unwrap();
    let input_fp = PathBuf::from("tests/test_data/input_0.vcf");
    let output_fp = PathBuf::from("tests/test_data/actual_out_0.vcf");
    let expected_out_fp = PathBuf::from("tests/test_data/expected_out_0.vcf");

    let _ = process_one_vcf(&input_fp, &output_fp, &snps);
    
    let mut out_file = File::open(&output_fp).unwrap();
    let mut out_str = String::new();

    out_file.read_to_string(&mut out_str).unwrap_or_else(|why| {
        panic!("Could not read data in: {}", why);
    });
    
    let mut exp_out_file = File::open(expected_out_fp).unwrap();
    let mut exp_str = String::new();

    exp_out_file.read_to_string(&mut exp_str).unwrap_or_else(|why| {
        panic!("Could not read data in: {}", why);
    });

    assert_eq!(out_str, exp_str);
    
    let _ = remove_file(output_fp);
}

#[test]
fn test_full_run_1() {
    let snps = load_snps(&PathBuf::from("tests/test_data/snpdb.vcf")).unwrap();
    let input_fp = PathBuf::from("tests/test_data/input_1.vcf");
    let output_fp = PathBuf::from("tests/test_data/actual_out_1.vcf");
    let expected_out_fp = PathBuf::from("tests/test_data/expected_out_1.vcf");

    let _ = process_one_vcf(&input_fp, &output_fp, &snps);
    
    let mut out_file = File::open(&output_fp).unwrap();
    let mut out_str = String::new();

    out_file.read_to_string(&mut out_str).unwrap_or_else(|why| {
        panic!("Could not read data in: {}", why);
    });
    
    let mut exp_out_file = File::open(expected_out_fp).unwrap();
    let mut exp_str = String::new();

    exp_out_file.read_to_string(&mut exp_str).unwrap_or_else(|why| {
        panic!("Could not read data in: {}", why);
    });

    assert_eq!(out_str, exp_str);

    let _ = remove_file(output_fp);
}
