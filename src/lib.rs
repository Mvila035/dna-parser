pub mod onehot;
pub mod kmers;
pub mod random_sequence;
pub mod fasta_files;



use numpy::{IntoPyArray, PyArray, PyArray2};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use numpy::ndarray::Array2;
use numpy::ndarray::ArrayView;
use pyo3::types::PyModule;
use pyo3::PyResult;

use onehot::one_hot;
use kmers::make_kmers;
use random_sequence::random_seq;


#[pyfunction]
fn OneHot<'pyt>(py:  Python <'pyt>, seq: String) ->  &'pyt PyArray2<i8>{


    let  matrix= one_hot(&seq);
    let pyarray= matrix.into_pyarray(py);

    return pyarray
    
}


#[pyfunction]
fn MakeKmers(seq: String, k: i64) -> String {

    let return_seq= make_kmers(k,&seq);

    return return_seq
}

#[pyfunction]
fn RandomSeq(length: i64, seq_type: String) -> String {

    let return_seq= random_seq(length, &seq_type);
    return return_seq
}

#[pymodule]
fn dna_parser(_py: Python<'_>, m: &PyModule)-> PyResult<()> {

    m.add_function(wrap_pyfunction!(OneHot,m)?)?;
    m.add_function(wrap_pyfunction!(MakeKmers,m)?)?;
    m.add_function(wrap_pyfunction!(RandomSeq,m)?)?;


    Ok(())
}



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

