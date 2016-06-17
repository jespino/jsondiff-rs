use serde_json::value::Value;
use serde_json::builder::{ObjectBuilder};

use std::collections::BTreeMap;

type Error = String;

struct Matrix {
    data: Vec<Vec<f64>>
}

enum Sign {
    Positive,
    Negative,
    Zero
}

struct Length {
    sign: Sign,
    value: Value,
    position: usize,
    similarity: f64
}

impl Length {
    fn new(sign: Sign, value: Value, position: usize, similarity: f64) -> Length {
        Length {
            sign: sign,
            value: value,
            position: position,
            similarity: similarity,
        }
    }
}


impl Matrix {
    fn new(array1: &Value, array2: &Value) -> Matrix {
        assert!(array1.is_array());
        assert!(array2.is_array());

        let x = array1.as_array().unwrap().len();
        let y = array2.as_array().unwrap().len();

        let mut matrix = Matrix {data: vec![]};

        for _ in 0..x+1 {
            let mut row = vec![];
            for _ in 0..y+1 {
                row.push(0.0);
            }
            matrix.data.push(row);
        }

        for i in 1..x+1 {
            for j in 1..y+1 {
                let (_, s) = value_diff(&array1.as_array().unwrap()[i-1], &array2.as_array().unwrap()[j-1]);
                matrix.data[i][j] = f64::max(matrix.data[i][j-1], f64::max(matrix.data[i-1][j], matrix.data[i-1][j-1] + s));
            }
        }
        return matrix;
    }

    fn length(&mut self, array1: &Value, array2: &Value, i: usize, j: usize) -> Vec<Length> {
        assert!(array1.is_array());
        assert!(array2.is_array());

        if i > 0 && j > 0 {
            let (d, s) = value_diff(&array1.as_array().unwrap()[i-1], &array2.as_array().unwrap()[j-1]);
            if s > 0.0 && self.data[i][j] == self.data[i-1][j-1] + s {
                let mut result = vec![];
                for annotation in self.length(array1, array2, i-1, j-1) {
                    result.push(annotation);
                }
                result.push(Length::new(Sign::Zero, d, j-1, s));
                return result;
            }
        }

        if j > 0 && (i == 0 || self.data[i][j-1] >= self.data[i-1][j]) {
            let mut result = vec![];
            for annotation in self.length(array1, array2, i, j-1) {
                result.push(annotation);
            }
            result.push(Length::new(Sign::Positive, array2.as_array().unwrap()[j-1].clone(), j-1, 0.0));
            return result;
        }

        if i > 0 && (j == 0 || self.data[i][j-1] < self.data[i-1][j]) {
            let mut result = vec![];
            for annotation in self.length(array1, array2, i-1, j) {
                result.push(annotation);
            }
            result.push(Length::new(Sign::Negative, array1.as_array().unwrap()[i-1].clone(), i-1, 0.0));
            return result;
        }
        return vec![];
    }
}


fn array_diff(array1: &Value, array2: &Value) -> (Value, f64) {
    assert!(array1.is_array());
    assert!(array2.is_array());
    // LCS
    let mut matrix = Matrix::new(array1, array2);

    let mut inserted = vec![];
    let mut deleted = vec![];
    let mut changed = BTreeMap::new();
    let mut tot_s = 0.0;

    for length in matrix.length(&array1, &array2, array1.as_array().unwrap().len(), array2.as_array().unwrap().len()) {
        match length.sign {
            Sign::Positive => inserted.push((length.position, length.value)),
            Sign::Negative => deleted.insert(0, (length.position, length.value)),
            Sign::Zero => if length.similarity < 1.0 { changed.insert(format!("{}", length.position), length.value); },
        }
        tot_s += length.similarity;
    }

    let tot_n = array1.as_array().unwrap().len() + inserted.len();
    let s = if tot_n == 0 { 1.0 } else { tot_s / tot_n as f64};

    if s == 0.0 { return (array2.clone(), s) }
    if s == 1.0 { return (ObjectBuilder::new().unwrap(), s) }

    if inserted.len() == 0 && deleted.len() == 0 && changed.len() == 0 {
        return (Value::Object(BTreeMap::new()), s)
    }

    let mut diffs = ObjectBuilder::new();
    if inserted.len() > 0 {
        diffs = diffs.insert("inserted", inserted);
    }
    if deleted.len() > 0 {
        diffs = diffs.insert("deleted", deleted);
    }
    if changed.len() > 0 {
        diffs = diffs.insert("changed", changed);
    }

    return (diffs.unwrap(), s);
}


fn obj_diff(obj1: &Value, obj2: &Value) -> (Value, f64) {
    assert!(obj1.is_object());
    assert!(obj2.is_object());

	let mut removed = BTreeMap::new();
	let mut nremoved = 0;
	let mut nadded = 0;
	let mut nmatched = 0;
	let mut smatched = 0.0;
	let mut added: BTreeMap<String, Value> = BTreeMap::new();
	let mut changed: BTreeMap<String, Value> = BTreeMap::new();

	for (k, v) in obj1.as_object().unwrap().iter() {
		match obj2.as_object().unwrap().get(k) {
			Some(w) => {
				nmatched += 1;
				let (d, s) = value_diff(v, w);
                if s < 1.0 {
                    changed.insert(k.clone(), d);
                }
                smatched += 0.5 + 0.5 * s;
			},
			None => {
				nremoved += 1;
				removed.insert(k, v.clone());
			}
		}
	}

	for (k, v) in obj2.as_object().unwrap().iter() {
	    if !obj1.as_object().unwrap().contains_key(k) {
	        nadded += 1;
	        added.insert(k.clone(), v.clone());
		}
	}

	let n_tot = nremoved + nmatched + nadded;
	let s = if n_tot != 0 { smatched / n_tot as f64 } else { 1.0 };

    let mut diffs = ObjectBuilder::new();
    if added.len() == 0 && removed.len() == 0 && changed.len() == 0 {
        return (Value::Object(BTreeMap::new()), s)
    }

    if added.len() > 0 {
        diffs = diffs.insert("added", added);
    }
    if removed.len() > 0 {
        diffs = diffs.insert("removed", removed);
    }
    if changed.len() > 0 {
        diffs = diffs.insert("changed", changed);
    }

    return (diffs.unwrap(), s)
}


pub fn value_diff(a: &Value, b: &Value) -> (Value, f64){
	if a.is_object() && b.is_object() {
	    return obj_diff(a, b);
    }
    if a.is_array() && b.is_array() {
        return array_diff(a, b);
    }
	if a != b {
	    return (b.clone(), 0.0)
    }
    return (Value::Object(BTreeMap::new()), 1.0)
}
