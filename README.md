# rust-pangenome-bedtools-compare
 - rust pangenome variants compare
 - compares two pangenome vcf files and then outputs the comparison and also the additional region which was at the same stretch in the one pangenome but was not present in the other.
 - write down the fasta for the same.
 - similar to bedtools intersect for vcf but for pangenomes.
 - please see the last commit message and if it says compiled binary then it is completed or else still in development version.


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
 - to run the binary

 ```
 ./target/debug/rust-pangenome-vcf \
   ./sample-files/a_vcfSVtest.vcf \
      ./sample-files/b_vcfSVtest.vcf \
            ./sample-files/sample.fasta \



 common_variant.txt : These have common start and end points in both the vcf and the corresponding sequences.
 chr28	15	43	15	43	A	DEL	A	DEL	20.25	20.25	GACTGACGTAGACATGACATGACTGACT	GACTGACGTAGACATGACATGACTGACT

 startpoint_variants.txt: These have the common start point but differ in the end points in both the vcf files aligned,
 so it tabulates how much is the difference and the sequence for the difference so that it can be used
 for the binding site affinity assay.
 chr28	10	42	10	40	C	DEL	G	T	CA	TGACTGACTGACGTAGACATGACATGACTGAC	TGACTGACTGACGTAGACATGACATGACTG
 chr28	15	43	15	42	A	DEL	C	INS	T	GACTGACGTAGACATGACATGACTGACT	GACTGACGTAGACATGACATGACTGAC
 chr28	20	41	20	45	A	DEL	A	C	CTGA	ACGTAGACATGACATGACTGA	ACGTAGACATGACATGACTGACTGA

 endpoint_variants.txt: These have common end points in both the vcf file files aligned to the same end points
 but differ in the start coordinate, so it also tabulates how much is the difference and the sequence
 for the difference so that it can be used for the binding site affinity assay.
 chr28	5	40	10	40	G	DEL	G	T	ATGAC	ATGACTGACTGACTGACGTAGACATGACATGACTG	TGACTGACTGACGTAGACATGACATGACTG
 chr28	10	42	15	42	C	DEL	C	INS	TGACT	TGACTGACTGACGTAGACATGACATGACTGAC	GACTGACGTAGACATGACATGACTGAC

```


Gaurav Sablok
