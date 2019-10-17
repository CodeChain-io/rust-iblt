use std::collections::HashMap;

use iblt::IBLT;

macro_rules! encode_and_decode {
    ($target: expr, $size: expr, $hash_count: expr) => {{
        let mut iblt = IBLT::new($size, $hash_count);
        for (k, v) in $target.iter() {
            iblt.insert(k, v).unwrap();
        }
        let (result, _) = iblt.decode().unwrap();

        result
    }};
}

#[test]
fn single_value_test() {
    let mut target = HashMap::new();
    target.insert("foo".to_string(), "bar".to_string());

    assert_eq!(target, encode_and_decode!(target, 10, 3));
}

#[test]
fn multiple_value_test() {
    let mut target = HashMap::new();
    target.insert("378b8bc3".to_string(), "4725a63a".to_string());
    target.insert("3f84ef5a".to_string(), "fbfc32d3".to_string());
    target.insert("8afb596f".to_string(), "40abfd05".to_string());
    target.insert("ec276396".to_string(), "a866db2e".to_string());
    target.insert("e785c851".to_string(), "0603063c".to_string());
    target.insert("90dfc042".to_string(), "17d64a66".to_string());
    target.insert("8f86f01d".to_string(), "747523d9".to_string());
    target.insert("3d61cdb4".to_string(), "d34c0bd7".to_string());
    target.insert("cdc07339".to_string(), "1831fdde".to_string());
    target.insert("3cb89aed".to_string(), "6aeec1ed".to_string());

    assert_eq!(target, encode_and_decode!(target, 15, 3));
}
