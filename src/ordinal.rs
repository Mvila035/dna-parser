use numpy::ndarray::{Array,Array1,Array2};
use numpy::{IntoPyArray, PyArray1, PyArray2};
use pyo3::prelude::*;
use crate::fasta_files::get_dna_only;



pub fn ordinal(sequence: &str)-> Array1<f32>{

    let ncols= sequence.len();
    let mut vec= Array1::<f32>::zeros(ncols);
   
    for elements in sequence.chars().enumerate() {

        let (index, charac)= elements;
        match charac {

            'A' => vec[index]= 0.25,
            'C' => vec[index]= 0.50,
            'G' => vec[index]= 0.75,
            'T' => vec[index]= 1.0,
            'U' => vec[index]= 1.0,
            'a' => vec[index]= 0.25,
            'c' => vec[index]= 0.50,
            'g' => vec[index]= 0.75,
            't' => vec[index]= 1.0,
            'u' => vec[index]= 1.0,
            _ => vec[index]= 0.0


        }       

    }
    let matrix= Array::from(vec);

    matrix
}

#[pyfunction]
pub fn batch_ordinal_encoding<'pyt>(py:  Python <'pyt>, sequences: Vec<&str>) ->  &'pyt PyArray2<f32>{

    let nrows= sequences.len();
    let ncols=sequences[0].len();
    let mut matrix = Array2::<f32>::zeros((nrows,ncols));

    for elements in sequences.iter().zip(matrix.rows_mut()){

        let (seq,mut current_row)= elements;
        let encoding= ordinal(seq);
        current_row.assign(&encoding);
    }

    let py_array= matrix.into_pyarray(py);
    py_array

}



#[pyfunction]
pub fn ordinal_from_fasta<'pyt>(py: Python  <'pyt>,path: String) -> &'pyt PyArray2<f32>{

    let sequences= get_dna_only(&path);
    let nrows= sequences.len();
    let ncols=sequences[0].len();
    let mut matrix = Array2::<f32>::zeros((nrows,ncols));

    for elements in sequences.iter().zip(matrix.rows_mut()){

        let (seq,mut current_row)= elements;
        let encoding= ordinal(seq);
        current_row.assign(&encoding);
    }


    let py_array= matrix.into_pyarray(py);

    py_array

}


#[pyfunction]
pub fn ordinal_encoding<'pyt>(py:  Python <'pyt>, seq: &str) ->  &'pyt PyArray1<f32>{


    let  matrix= ordinal(seq);
    let pyarray= matrix.into_pyarray(py);

    return pyarray
    
}