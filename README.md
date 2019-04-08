# IBLT

An Invertible Bloom Lookup Table implementation in the Rust programming language.

`IBLT` uses bincode for serialization/deserialization of key and values, so both key and value type must implement `Serialize` and `Deserialize`. If two serialized values have different lengths, zero padding is added at the end of the shorter value for xor operation.

A hash function can be specified by creating an `IBLT` with the `with_hasher` function. The hash value for `hash_sum` is calculated as `hash(val)`, and the index is calculated as `hash(hash(val)), hash(hash(hash(val))), ...`. If the hasher is not specified, `DefaultHasher` from `std::collections::hash_map` is used.

## Example
```rust
  /// encoding
  let mut iblt = IBLT::new(10, 3);
  target.insert("378b8bc3".to_string(), "4725a63a".to_string());
  target.insert("3f84ef5a".to_string(), "fbfc32d3".to_string());
  target.insert("8afb596f".to_string(), "40abfd05".to_string());
  target.insert("ec276396".to_string(), "a866db2e".to_string());
  target.insert("e785c851".to_string(), "0603063c".to_string());

  /// decoding
  match iblt.decode() {
    Ok((left, right)) => { ... },
    Err(e) => { ... },
  }
```

## TODO
* Subtraction and set reconciliation
* Unsafe serialize/deserialize with std
