# IBLT

Invertible bloom lookup table implementation in rust.

`IBLT` uses bincode for serialization/deserialization of key and values, so both key and value type must implement `Serialize` and `Deserialize`. If two serialized values have different length, zero padding is added at the end of shorter value for xor operation.

Hash function can be specified by creating `IBLT` with `with_hasher` function. Hash value for `hash_sum` is calculated as `hash(val)`, and index is calculated as `hash(hash(val)), hash(hash(hash(val))), ...`. If hasher is not specified, `DefaultHasher` from `std::collections::hash_map` is used.

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
