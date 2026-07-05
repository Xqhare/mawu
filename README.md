# `Mawu`

A JSON and CSV serialization and deserialization library written in rust.

`Mawu`, named after the ancient creator goddess `Mawu` in West African mythology, offers a JSON and CSV serialization and deserialization library implementing the rfc4180, rfc8259 and the ECMA-404 standard.

`Mawu` is a zero dependency library and supports 64bit systems only.

It follows my "All code written by me or part of rust's standard library and libc" philosophy.
You can learn more about that [here](https://blog.xqhare.net/posts/why_solve_problems/).

## Features

- _**No dependencies**_: All code is written by me or part of std.
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

## Roadmap

`Mawu` uses my [nomos](https://github.com/xqhare/nomos) project management system.

The roadmap for this project can be found in the [nomos.md](nomos.md) file.

All nomos files follow the syntax defined [here](https://github.com/Xqhare/nomos/blob/master/spec/).

## Naming

As with all my projects, `Mawu` is named after an ancient deity.
Learn more about my naming scheme [here](https://blog.xqhare.net/posts/explaining_the_pantheon/).

## Using `Mawu`

Start by adding this repository to your `Cargo.toml`.

```toml
[dependencies]
mawu = { git = "https://github.com/Xqhare/mawu" }
```

### Reading JSON

`Mawu` now returns `athena::XffValue` directly when parsing JSON.

```rust
use mawu::read::json;

# std::fs::write("example.json", "{}").unwrap();
let path_to_file = "example.json";
let xff_value = json(path_to_file).unwrap();
if xff_value.is_object() {
    for (key, value) in xff_value.into_object().unwrap().iter() {
        println!("{}: {}", key, value);
    }
}
# std::fs::remove_file("example.json").unwrap();
```

### Reading CSV

CSV support is gated behind the `csv` feature flag.

CSV data is returned wrapped in a `MawuValue`.

```rust
# #[cfg(feature = "csv")]
# {
use mawu::read::csv_headed;

# std::fs::write("example.csv", "a,b\n1,2").unwrap();
let path_to_file = "example.csv";
let csv_value = csv_headed(path_to_file).unwrap();
if csv_value.is_csv_object() {
    for row in csv_value.as_csv_object().unwrap() {
        for (key, value) in row {
            println!("{}: {}", key, value);
        }
    }
}
# std::fs::remove_file("example.csv").unwrap();
# }
```

### Writing

`Mawu` has a unified writing API, use the `MawuContents` enum to wrap your data.

```rust
use mawu::{write, write_pretty, MawuContents, MawuValue};
use athena::XffValue;

// Writing JSON
let xff_val = XffValue::from(vec![1, 2, 3]);
write("output.json", MawuContents::Json(xff_val)).unwrap();

// Writing CSV
# #[cfg(feature = "csv")]
# {
let csv_val = MawuValue::new_csv_array(); // ... fill your CSV data
write("output.csv", MawuContents::Csv(csv_val)).unwrap();
# }
# std::fs::remove_file("output.json").unwrap();
# #[cfg(feature = "csv")]
# std::fs::remove_file("output.csv").unwrap();
```

## `MawuValue` vs `XffValue`
- **`XffValue`** (from the `athena` crate) is the primary data structure for JSON.
- **`MawuValue`** is a specialized wrapper for CSV data, holding either `CSVObject` (headed) or `CSVArray` (headless), where each field is an `XffValue`.

## `MawuContents`
The `MawuContents` enum unifies `XffValue` and `MawuValue` for the `write` and `write_pretty` functions.
- `MawuContents::Json(XffValue)`
- `MawuContents::Csv(MawuValue)`
