

pub mod onehot;
pub mod kmers;
pub mod random_sequence;
pub mod fasta_files;


#[cfg(test)]
mod tests {

    use crate::random_sequence::random_seq;
    use crate::fasta_files::write_to_file;
    
    #[test]
    fn random_test() {
        let len: i64 = 100;
        let seq= random_seq(len, "dna");

        println!("{},{}", seq.chars().count(), seq);

        let test= len as usize;
        assert_eq!(seq.chars().count(),test);
    }

    #[test]
    fn write_test() {

        let metadata: Vec<String>= ["lol".to_string(), "abcd".to_string(), "okay".to_string()].to_vec();
        let data: Vec<String> = ["bonjour".to_string(), "oui".to_string(), "yes".to_string()].to_vec();
        let path: String = "/Users/matthieuvilain/desktop/test.fasta".to_string();

        write_to_file(&path, &data, &metadata);

    }

}

