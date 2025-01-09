# rust-pangenome-bedtools-compare
 - rust pangenome variants compare
 - compares two pangenome vcf files and then outputs the comparison and also the additional region which was at the same stretch in the one pangenome but was not present in the other.
 - write down the fasta for the same. 
 - similar to bedtools intersect for vcf but for pangenomes. 
 - general note: Incase of Golang and RUST, please see the last commit message and if it says compiled binary then it is completed or else still in development version.


 ```
 cargo build 
 
 ```

 ```
 gauravsablok Usage: rust-pangenome-vcf <VCF1> <VCF2> <FASTA>

 Arguments:
  <VCF1>   please provide the path to the first VCF file
  <VCF2>   please provide the path to the second VCF file
  <FASTA>  please provide the path to the fasta file

 Options:
  -h, --help     Print help
  -V, --version  Print version  

 ```
 
 Gaurav Sablok
