use numpy::ndarray::ArrayView;
use numpy::ndarray::{Array1,Array2};
use numpy::ndarray::array;

pub fn one_hot(sequence: &str)-> Array2<i8> {

    let seq= sequence.to_lowercase();
    let nrows= sequence.len();
    let ncols= 4 as usize;
    let mut matrix= Array2::<i8>::zeros((nrows,ncols));

    //define values for one hot encoding
    for elements in seq.chars().zip(matrix.rows_mut()) {

        let (charac, mut current_row)= elements;
        match charac {

            'a' => current_row.assign(&ArrayView::from(&[ 1,  0,  0,  0])),
            'c' => current_row.assign(&ArrayView::from(&[ 0,  1,  0,  0])),
            'g' => current_row.assign(&ArrayView::from(&[ 0,  0,  1,  0])),
            't' => current_row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            'u' => current_row.assign(&ArrayView::from(&[ 0,  0,  0,  1])),
            _ => continue
        }
    }
    return matrix
}

pub fn ordinal(sequence: &str)-> Array1<f32>{

    let seq= sequence.to_lowercase();
    let ncols= seq.len();
    let mut matrix= Array1::<f32>::zeros(ncols);

    for elements in seq.chars().zip(matrix.columns_mut()) {

        let (charac, mut current_col)= elements;

        match charac {

            'a' => current_col.assign(&array![0.25]),
            'c' => current_col.assign(&array![0.50]),
            'g' => current_col.assign(&array![0.75]),
            't' => current_col.assign(&array![1.0]),
            'u' => current_col.assign(&array![1.0]),
            _ => continue


        }
        

    }

    matrix
}