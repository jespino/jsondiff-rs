use serde_json::value::Value;
use lcs::{value_diff};


pub fn compare(a: &Value, b: &Value) -> Value {
    value_diff(a, b).0
}

#[cfg(test)]
mod tests {
    use super::compare;
    use serde_json::value::Value;
    use serde_json::builder::{ArrayBuilder, ObjectBuilder};
    use std::collections::BTreeMap;
    use test::Bencher;

    #[test]
    fn test_equality() {
        let empty = Value::Object(BTreeMap::new());
        assert_eq!(empty, compare(&Value::U64(1), &Value::U64(1)));
        assert_eq!(empty, compare(&Value::Bool(true), &Value::Bool(true)));
        assert_eq!(empty, compare(&Value::String("abc".to_string()), &Value::String("abc".to_string())));
        assert_eq!(empty, compare(&ArrayBuilder::new().push(1).push(2).unwrap(),
                                  &ArrayBuilder::new().push(1).push(2).unwrap()));
        assert_eq!(empty, compare(&ObjectBuilder::new().insert("a", 1).insert("b", 2).unwrap(),
                                  &ObjectBuilder::new().insert("a", 1).insert("b", 2).unwrap()));
        assert_eq!(empty, compare(&Value::Array(vec![]), &Value::Array(vec![])));
        assert_eq!(empty, compare(&Value::Null, &Value::Null));
        assert_eq!(empty, compare(&Value::Object(BTreeMap::new()), &Value::Object(BTreeMap::new())));
    }

    #[test]
    fn test_simple_differences() {
        assert_eq!(Value::U64(2), compare(&Value::U64(1), &Value::U64(2)));
        assert_eq!(Value::Bool(false), compare(&Value::Bool(true), &Value::Bool(false)));
        assert_eq!(Value::String("def".to_string()), compare(&Value::String("abc".to_string()), &Value::String("def".to_string())));
        assert_eq!(ArrayBuilder::new().push(3).push(4).unwrap(),
                   compare(&ArrayBuilder::new().push(1).push(2).unwrap(),
                           &ArrayBuilder::new().push(3).push(4).unwrap()));
        assert_eq!(ObjectBuilder::new().insert_object("changed", |builder| builder.insert("a", 3).insert("b", 4)).unwrap(),
                   compare(&ObjectBuilder::new().insert("a", 1).insert("b", 2).unwrap(),
                           &ObjectBuilder::new().insert("a", 3).insert("b", 4).unwrap()));
    }

    #[bench]
    fn bench_compare_arrays(b: &mut Bencher) {
        let array1 = ArrayBuilder::new().push(1).push(2).push(3).push(4).unwrap();
        let array2 = ArrayBuilder::new().push(3).push(4).push(5).push(6).unwrap();
        b.iter(|| compare(&array1, &array2));
    }

    #[bench]
    fn bench_compare_dicts(b: &mut Bencher) {
        let obj1 = ObjectBuilder::new().insert("a", 1).insert("b", 2).insert("c", 3).insert("d", 4).unwrap();
        let obj2 = ObjectBuilder::new().insert("c", 3).insert("d", 4).insert("e", 5).insert("f", 6).unwrap();
        b.iter(|| compare(&obj2, &obj1));
    }
}
