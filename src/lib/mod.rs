//! A Json (LCS based) difference library
//!
//! This library is an implementation in Rust of the LCS (Longuest Common
//! Subsequence) algorithm for finding differences between json data.
//!
//! This implementation is based on the Python [jsondiff] library.
//!
//! [jsondiff]: https://github.com/ZoomerAnalytics/jsondiff

#![warn(missing_docs)]
#![feature(test)]
extern crate serde_json;
extern crate test;

mod lcs;

use serde_json::value::Value;
use lcs::{value_diff};


/// Calculate the differences between to serde_json::Value objects
///
/// # Example
///
/// ## Equal json values
///
/// ```
/// # extern crate serde_json;
/// # extern crate jsondiff;
/// # use serde_json::builder::ArrayBuilder;
/// # use serde_json::builder::ObjectBuilder;
/// # use jsondiff::diff;
/// # fn main() {
/// let array1 = serde_json::builder::ArrayBuilder::new().push(1).push(2).unwrap();
/// let array2 = serde_json::builder::ArrayBuilder::new().push(1).push(2).unwrap();
/// let differences = jsondiff::diff(&array1, &array2);
/// // differences == {}
/// assert_eq!(differences, ObjectBuilder::new().unwrap())
/// # }
/// ```
///
/// ## Different json values
///
/// ```
/// # extern crate serde_json;
/// # extern crate jsondiff;
/// # use serde_json::builder::ArrayBuilder;
/// # use serde_json::builder::ObjectBuilder;
/// # use jsondiff::diff;
/// # fn main() {
/// let array1 = serde_json::builder::ArrayBuilder::new().push(1).push(2).unwrap();
/// let array2 = serde_json::builder::ArrayBuilder::new().push(2).unwrap();
/// let differences = jsondiff::diff(&array1, &array2);
/// // differences == {"deleted": [[0, 1]]}
/// assert_eq!(differences,
///     ObjectBuilder::new().insert("deleted",
///         ArrayBuilder::new().push(
///             ArrayBuilder::new().push(0).push(1).unwrap()
///         ).unwrap()
///     ).unwrap());
/// # }
/// ```
pub fn diff(a: &Value, b: &Value) -> Value {
    value_diff(a, b).0
}

/// Calculate the similarity between to serde_json::Value objects
///
/// # Example
///
/// ## Equal json values
///
/// ```
/// # extern crate serde_json;
/// # extern crate jsondiff;
/// # use serde_json::builder::ArrayBuilder;
/// # use jsondiff::similarity;
/// # fn main() {
/// let array1 = serde_json::builder::ArrayBuilder::new().push(1).push(2).push(3).unwrap();
/// let array2 = serde_json::builder::ArrayBuilder::new().push(1).push(2).push(3).unwrap();
/// let similarity = jsondiff::similarity(&array1, &array2);
/// assert_eq!(similarity, 1.0);
/// # }
/// ```
///
/// ## Different json values
///
/// ```
/// # extern crate serde_json;
/// # extern crate jsondiff;
/// # use serde_json::builder::ArrayBuilder;
/// # use jsondiff::similarity;
/// # fn main() {
/// let array1 = serde_json::builder::ArrayBuilder::new().push(1).push(2).push(3).unwrap();
/// let array2 = serde_json::builder::ArrayBuilder::new().push(2).push(3).push(4).unwrap();
/// let similarity = jsondiff::similarity(&array1, &array2);
/// assert_eq!(similarity, 0.5);
/// # }
/// ```
pub fn similarity(a: &Value, b: &Value) -> f64 {
    value_diff(a, b).1
}

#[cfg(test)]
mod tests {
    use super::{diff, similarity};
    use serde_json::value::Value;
    use serde_json::builder::{ArrayBuilder, ObjectBuilder};
    use std::collections::BTreeMap;
    use test::Bencher;

    #[test]
    fn test_diff_equality() {
        let empty = Value::Object(BTreeMap::new());
        assert_eq!(empty, diff(&Value::U64(1), &Value::U64(1)));
        assert_eq!(empty, diff(&Value::Bool(true), &Value::Bool(true)));
        assert_eq!(empty, diff(&Value::String("abc".to_string()), &Value::String("abc".to_string())));
        assert_eq!(empty, diff(&ArrayBuilder::new().push(1).push(2).unwrap(),
                               &ArrayBuilder::new().push(1).push(2).unwrap()));
        assert_eq!(empty, diff(&ObjectBuilder::new().insert("a", 1).insert("b", 2).unwrap(),
                               &ObjectBuilder::new().insert("a", 1).insert("b", 2).unwrap()));
        assert_eq!(empty, diff(&Value::Array(vec![]), &Value::Array(vec![])));
        assert_eq!(empty, diff(&Value::Null, &Value::Null));
        assert_eq!(empty, diff(&Value::Object(BTreeMap::new()), &Value::Object(BTreeMap::new())));
    }

    #[test]
    fn test_simple_differences() {
        assert_eq!(Value::U64(2), diff(&Value::U64(1), &Value::U64(2)));
        assert_eq!(Value::Bool(false), diff(&Value::Bool(true), &Value::Bool(false)));
        assert_eq!(Value::String("def".to_string()), diff(&Value::String("abc".to_string()), &Value::String("def".to_string())));
        assert_eq!(ArrayBuilder::new().push(3).push(4).unwrap(),
                   diff(&ArrayBuilder::new().push(1).push(2).unwrap(),
                        &ArrayBuilder::new().push(3).push(4).unwrap()));
        assert_eq!(ObjectBuilder::new().insert_object("changed", |builder| builder.insert("a", 3).insert("b", 4)).unwrap(),
                   diff(&ObjectBuilder::new().insert("a", 1).insert("b", 2).unwrap(),
                        &ObjectBuilder::new().insert("a", 3).insert("b", 4).unwrap()));
    }

    #[test]
    fn test_similarity_for_equals() {
        assert_eq!(1.0, similarity(&Value::U64(1), &Value::U64(1)));
        assert_eq!(1.0, similarity(&Value::Bool(true), &Value::Bool(true)));
        assert_eq!(1.0, similarity(&Value::String("abc".to_string()), &Value::String("abc".to_string())));
        assert_eq!(1.0, similarity(&ArrayBuilder::new().push(1).push(2).unwrap(),
                                   &ArrayBuilder::new().push(1).push(2).unwrap()));
        assert_eq!(1.0, similarity(&ObjectBuilder::new().insert("a", 1).insert("b", 2).unwrap(),
                                   &ObjectBuilder::new().insert("a", 1).insert("b", 2).unwrap()));
        assert_eq!(1.0, similarity(&Value::Array(vec![]), &Value::Array(vec![])));
        assert_eq!(1.0, similarity(&Value::Null, &Value::Null));
        assert_eq!(1.0, similarity(&Value::Object(BTreeMap::new()), &Value::Object(BTreeMap::new())));
    }

    #[test]
    fn test_similarity_for_different_objects() {
        assert_eq!(0.0, similarity(&Value::U64(1), &Value::U64(2)));
        assert_eq!(0.0, similarity(&Value::Bool(true), &Value::Bool(false)));
        assert_eq!(0.0, similarity(&Value::String("abc".to_string()), &Value::String("def".to_string())));
        assert_eq!(0.5, similarity(&ArrayBuilder::new().push(1).push(2).push(3).unwrap(),
                                   &ArrayBuilder::new().push(2).push(3).push(4).unwrap()));
        assert_eq!(0.5, similarity(&ObjectBuilder::new().insert("a", 1).insert("b", 2).unwrap(),
                                   &ObjectBuilder::new().insert("a", 3).insert("b", 4).unwrap()));
    }

    #[bench]
    fn bench_diff_arrays(b: &mut Bencher) {
        let array1 = ArrayBuilder::new().push(1).push(2).push(3).push(4).unwrap();
        let array2 = ArrayBuilder::new().push(3).push(4).push(5).push(6).unwrap();
        b.iter(|| diff(&array1, &array2));
    }

    #[bench]
    fn bench_diff_dicts(b: &mut Bencher) {
        let obj1 = ObjectBuilder::new().insert("a", 1).insert("b", 2).insert("c", 3).insert("d", 4).unwrap();
        let obj2 = ObjectBuilder::new().insert("c", 3).insert("d", 4).insert("e", 5).insert("f", 6).unwrap();
        b.iter(|| diff(&obj2, &obj1));
    }
}
