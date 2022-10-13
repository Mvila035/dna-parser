use numpy::ndarray::{Array,Array1};
use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::thread;
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


#[allow(unused_must_use)]
#[pyfunction]
pub fn batch_ordinal_encoding<'pyt>(py:  Python <'pyt>, sequences: Vec<&str>, ) ->  &'pyt PyList{

    let (first,second)= sequences.split_at(sequences.len()/2);
    let mut list_2= Vec::new();
    let mut list = Vec::new();
    let py_list= PyList::empty(py);

    thread::scope(|s|{
    

        s.spawn(|| {
            
            for seq in second.into_iter(){

                let encoding= ordinal(seq);
                
                list_2.push(encoding);
                
            }

        });
        

        for seq in first.into_iter(){

            let encoding= ordinal(seq);
            
            list.push(encoding);
            
        }


       
    
       
    });

    for seq in list {

        py_list.append(seq.into_pyarray(py));
    }

    for seq_2 in list_2 {

        py_list.append(seq_2.into_pyarray(py));
    }
    
    py_list
    
}


#[allow(unused_must_use)]
#[pyfunction]
pub fn ordinal_from_fasta<'pyt>(py: Python  <'pyt>,path: String) -> &'pyt PyList{

    let sequences= get_dna_only(&path);
    let list = PyList::empty(py);

    for seq in sequences{

        let encoding= ordinal(&seq);
        
        list.append(encoding.into_pyarray(py));
        
    }

    
    list
}


#[pyfunction]
pub fn ordinal_encoding<'pyt>(py:  Python <'pyt>, seq: &str) ->  &'pyt PyArray1<f32>{


    let  matrix= ordinal(seq);
    let pyarray= matrix.into_pyarray(py);

    return pyarray
    
}