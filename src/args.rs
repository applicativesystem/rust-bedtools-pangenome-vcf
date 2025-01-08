use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct VcfArgs {
    /// please provide the path to the first VCF file
    pub vcf1: String,
    /// please provide the path to the second VCF file
    pub vcf2: String,
    /// please provide the path to the fasta file.
    pub fasta: String,
}
