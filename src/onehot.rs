use numpy::ndarray::ArrayView;
use numpy::ndarray::Array2;
use numpy::{IntoPyArray, PyArray2};
use pyo3::prelude::*;


fn one_hot(sequence: &str)-> Array2<i8> {

    let nrows= sequence.len();
    let ncols= 4 as usize;
    let mut matrix= Array2::<i8>::zeros((nrows,ncols));

    //define values for one hot encoding
    for elements in sequence.chars().zip(matrix.rows_mut()) {

        let (charac, mut current_row)= elements;
        match charac {

            'A' => current_row.assign(&ArrayView::from(&[ 1,  0,  0,  0])),
            'C' => current_row.assign(&ArrayView::from(&[ 0,  1,  0,  0])),
            'G' => current_row.assign(&ArrayView::from(&[ 0,  0,  1,  0])),
            'T' => current_row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'U' => current_row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'a' => current_row.assign(&ArrayView::from(&[ 1,  0,  0,  0])),
            'c' => current_row.assign(&ArrayView::from(&[ 0,  1,  0,  0])),
            'g' => current_row.assign(&ArrayView::from(&[ 0,  0,  1,  0])),
            't' => current_row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'u' => current_row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            _ => current_row.assign(&ArrayView::from(&[ 0,  0,  0,  0])),
        }
    }
    return matrix
}

#[pyfunction]
pub fn onehot_encoding<'pyt>(py:  Python <'pyt>, seq: String) ->  &'pyt PyArray2<i8>{


    let  matrix= one_hot(&seq);
    let pyarray= matrix.into_pyarray(py);

    return pyarray
    
}