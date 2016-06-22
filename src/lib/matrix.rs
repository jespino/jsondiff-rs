use serde_json::value::Value;
use lcs::value_diff;

type Matrix = Vec<Vec<f64>>;

pub enum Sign {
    Positive,
    Negative,
    Zero
}

pub struct Length {
    pub sign: Sign,
    pub value: Value,
    pub position: usize,
    pub similarity: f64
}

pub fn length(array1: &Value, array2: &Value) -> Vec<Length> {
    assert!(array1.is_array());
    assert!(array2.is_array());

    let matrix = init_matrix(array1, array2);
    rec_length(&matrix, array1, array2, array1.as_array().unwrap().len(), array2.as_array().unwrap().len())
}


fn init_matrix(array1: &Value, array2: &Value) -> Matrix {
    let x = array1.as_array().unwrap().len();
    let y = array2.as_array().unwrap().len();

    let mut matrix = Matrix::with_capacity(x+1);

    for _ in 0..x+1 {
        let mut row = Vec::with_capacity(y+1);
        for _ in 0..y+1 {
            row.push(0.0);
        }
        matrix.push(row);
    }

    for i in 1..x+1 {
        for j in 1..y+1 {
            let (_, s) = value_diff(&array1.as_array().unwrap()[i-1], &array2.as_array().unwrap()[j-1]);
            matrix[i][j] = f64::max(matrix[i][j-1], f64::max(matrix[i-1][j], matrix[i-1][j-1] + s));
        }
    }
    matrix
}


fn rec_length(matrix: &Matrix, array1: &Value, array2: &Value, i: usize, j: usize) -> Vec<Length> {
    if i > 0 && j > 0 {
        let (d, s) = value_diff(&array1.as_array().unwrap()[i-1], &array2.as_array().unwrap()[j-1]);
        if s > 0.0 && matrix[i][j] == matrix[i-1][j-1] + s {
            let mut result = rec_length(matrix, array1, array2, i-1, j-1);
            result.push(Length {
                sign: Sign::Zero,
                value: d,
                position: j-1,
                similarity: s
            });
            return result;
        }
    }

    if j > 0 && (i == 0 || matrix[i][j-1] >= matrix[i-1][j]) {
        let mut result = rec_length(matrix, array1, array2, i, j-1);
        result.push(Length {
            sign: Sign::Positive,
            value: array2.as_array().unwrap()[j-1].clone(),
            position: j-1,
            similarity: 0.0
        });
        return result;
    }

    if i > 0 && (j == 0 || matrix[i][j-1] < matrix[i-1][j]) {
        let mut result = rec_length(matrix, array1, array2, i-1, j);
        result.push(Length {
            sign: Sign::Negative,
            value: array1.as_array().unwrap()[i-1].clone(),
            position: i-1,
            similarity: 0.0
        });
        return result;
    }
    return vec![];
}
