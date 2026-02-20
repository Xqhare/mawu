# Mawu
A JSON and CSV serialization and deserialization library written in rust.

Mawu, named after the ancient creator goddess Mawu in West African mythology, offers a JSON and CSV serialization and deserialization library implementing the rfc4180, rfc8259 and the ECMA-404 standard.

Mawu is a zero dependency library and supports 64bit systems only.

***This is a hobbyist repo badly reinventing the wheel and not ready for production use.*** 

## Features
- Simple
- Type aware
- Supports both CSV and JSON
- Reading and writing
- Write pretty with custom spacing
- Supports CSV files with or without header
- Supports missing or not provided values
- Fully documented
- Tries to stay as close to the rfc4180, rfc8259 and ECMA-404 standard as possible for maximum interoperability
- Actually written by a human

## Using Mawu
Start by adding this repository to your `Cargo.toml`.
```toml
[dependencies]
mawu = { git = "https://github.com/Xqhare/mawu" }
```

### Reading JSON
Mawu now returns `athena::XffValue` directly when parsing JSON.

```rust
use mawu::read::json;

let path_to_file = "example.json";
let xff_value = json(path_to_file).unwrap();
if xff_value.is_object() {
    for (key, value) in xff_value.into_object().unwrap().iter() {
        println!("{}: {}", key, value);
    }
}
```

### Reading CSV
CSV data is returned wrapped in a `MawuValue`.

```rust
use mawu::read::csv_headed;

let path_to_file = "example.csv";
let csv_value = csv_headed(path_to_file).unwrap();
if csv_value.is_csv_object() {
    for row in csv_value.as_csv_object().unwrap() {
        for (key, value) in row {
            println!("{}: {}", key, value);
        }
    }
}
```

### Writing
To maintain a unified writing API, use the `MawuContents` enum to wrap your data.

```rust
use mawu::{write, write_pretty, MawuContents, MawuValue};
use athena::XffValue;

// Writing JSON
let xff_val = XffValue::from(vec![1, 2, 3]);
write("output.json", MawuContents::Json(xff_val)).unwrap();

// Writing CSV
let csv_val = MawuValue::new_csv_array(); // ... fill your CSV data
write("output.csv", MawuContents::Csv(csv_val)).unwrap();
```

## `MawuValue` vs `XffValue`
- **`XffValue`** (from the `athena` crate) is the primary data structure for JSON.
- **`MawuValue`** is a specialized wrapper for CSV data, holding either `CSVObject` (headed) or `CSVArray` (headless), where each field is an `XffValue`.

## `MawuContents`
The `MawuContents` enum unifies `XffValue` and `MawuValue` for the `write` and `write_pretty` functions.
- `MawuContents::Json(XffValue)`
- `MawuContents::Csv(MawuValue)`
