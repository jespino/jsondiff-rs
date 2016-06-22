use serde_json::value::Value;
use serde_json::builder::{ObjectBuilder};

use std::collections::BTreeMap;

use matrix;

fn array_diff(array1: &Value, array2: &Value) -> (Value, f64) {
    assert!(array1.is_array());
    assert!(array2.is_array());

    let mut inserted = vec![];
    let mut deleted = vec![];
    let mut changed = BTreeMap::new();
    let mut tot_s = 0.0;

    // LCS
    for length in matrix::length(array1, array2) {
        match length.sign {
            matrix::Sign::Positive => inserted.push((length.position, length.value)),
            matrix::Sign::Negative => deleted.insert(0, (length.position, length.value)),
            matrix::Sign::Zero => if length.similarity < 1.0 { changed.insert(format!("{}", length.position), length.value); },
        }
        tot_s += length.similarity;
    }

    let tot_n = array1.as_array().unwrap().len() + inserted.len();
    let s = if tot_n == 0 { 1.0 } else { tot_s / tot_n as f64};

    if s == 0.0 { return (array2.clone(), s) }
    if s == 1.0 { return (ObjectBuilder::new().unwrap(), s) }

    if inserted.is_empty() && deleted.is_empty() && changed.is_empty() {
        return (Value::Object(BTreeMap::new()), s)
    }

    let mut diffs = ObjectBuilder::new();
    if !inserted.is_empty() {
        diffs = diffs.insert("inserted", inserted);
    }
    if !deleted.is_empty() {
        diffs = diffs.insert("deleted", deleted);
    }
    if !changed.is_empty() {
        diffs = diffs.insert("changed", changed);
    }

    (diffs.unwrap(), s)
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
    if added.is_empty() && removed.is_empty() && changed.is_empty() {
        return (Value::Object(BTreeMap::new()), s)
    }

    if !added.is_empty() {
        diffs = diffs.insert("added", added);
    }
    if !removed.is_empty() {
        diffs = diffs.insert("removed", removed);
    }
    if !changed.is_empty() {
        diffs = diffs.insert("changed", changed);
    }

    (diffs.unwrap(), s)
}


pub fn value_diff(a: &Value, b: &Value) -> (Value, f64){
    if a == b {
        return (Value::Object(BTreeMap::new()), 1.0);
    }
	if a.is_object() && b.is_object() {
	    return obj_diff(a, b);
    }
    if a.is_array() && b.is_array() {
        return array_diff(a, b);
    }
    (b.clone(), 0.0)
}

#[cfg(test)]
mod tests {
    use super::{array_diff, obj_diff};
    use test::Bencher;
    use serde_json::builder::{ArrayBuilder, ObjectBuilder};

    #[bench]
    fn bench_array_diff(b: &mut Bencher) {
        let array1 = ArrayBuilder::new().push(1).push(2).push(3).push(4).unwrap();
        let array2 = ArrayBuilder::new().push(3).push(4).push(5).push(6).unwrap();
        b.iter(|| array_diff(&array1, &array2));
    }

    #[bench]
    fn bench_obj_diff(b: &mut Bencher) {
        let obj1 = ObjectBuilder::new().insert("a", 1).insert("b", 2).insert("c", 3).insert("d", 4).unwrap();
        let obj2 = ObjectBuilder::new().insert("c", 3).insert("d", 4).insert("e", 5).insert("f", 6).unwrap();
        b.iter(|| obj_diff(&obj1, &obj2));
    }
}
