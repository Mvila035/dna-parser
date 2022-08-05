

pub mod onehot;
pub mod kmers;
pub mod random_sequence;
pub mod fasta_files;


#[cfg(test)]
mod tests {

    use crate::random_sequence::random_seq;
    
    #[test]
    fn random_test() {
        let len: i64 = 100;
        let seq= random_seq(len, "dna");

        println!("{},{}", seq.chars().count(), seq);

        let test= len as usize;
        assert_eq!(seq.chars().count(),test);
    }

}

