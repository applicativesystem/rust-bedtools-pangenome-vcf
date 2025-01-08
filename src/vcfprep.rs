/*
 * holding all the structs in the separate files so that they
 * can be easily called as a reference call in the result.
 *
 *
 * */
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct VCFRange {
    pub name: String,
    pub start: usize,
    pub end: usize,
    pub delorig: String,
    pub deltype: String,
    pub threshold: Box<f64>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Endpointcompare {
    pub name: String,
    pub start1: usize,
    pub end1: usize,
    pub start2: usize,
    pub end2: usize,
    pub delorig1: String,
    pub deltype1: String,
    pub threshold1: Box<f64>,
    pub delorig2: String,
    pub deltype2: String,
    pub threshold2: Box<f64>,
    pub difference: Box<usize>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Startpointcompare {
    pub name: String,
    pub start1: usize,
    pub end1: usize,
    pub start2: usize,
    pub end2: usize,
    pub delorig1: String,
    pub deltype1: String,
    pub threshold1: Box<f64>,
    pub delorig2: String,
    pub deltype2: String,
    pub threshold2: Box<f64>,
    pub difference: Box<usize>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Common {
    pub name: String,
    pub start1: usize,
    pub end1: usize,
    pub start2: usize,
    pub end2: usize,
    pub delorig1: String,
    pub deltype1: String,
    pub deltype2: String,
    pub delorig2: String,
    pub threshold1: f64,
    pub threshold2: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Fasta {
    pub header: String,
    pub sequence: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Fastasnatcher {
    pub name: String,
    pub start1: usize,
    pub end1: usize,
    pub start2: usize,
    pub end2: usize,
    pub delorig1: String,
    pub deltype1: String,
    pub threshold1: Box<f64>,
    pub delorig2: String,
    pub deltype2: String,
    pub threshold2: Box<f64>,
    pub sequenceadd: String,
    pub sequenceregion1: String,
    pub sequenceregion2: String,
}
