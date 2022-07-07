![](https://github.com/wigasper/snpsubtract/workflows/build/badge.svg) ![](https://github.com/wigasper/snpsubtract/workflows/deploy/badge.svg)

# snpsubtract

Subtracts common variants from VCF files using a VCF containing common variants as reference.

```
$ snpsubtract vcfs_in_directory snps_db.vcf vcfs_out_directory
```

Build with Rust

```bash
$ cargo build --release
```

Or just download a binary from the Releases page
