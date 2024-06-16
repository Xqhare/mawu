# Mawu
A simple JSON and CSV parsing library written in rust.

Mawu, named after the ancient creator goddess Mawu in West African mythology, offers a simple yet robust and reliable JSON and CSV parsing library implementing the rfc4180, rfc8259 and the ECMA-404 standard. It is not a zero dependency library, it's only dependency is `unicode-segmentation`.

Also, it should be said that this is a hobbyist repo and is probably not ready for production use.

## Features
- Simple and fast
- Type aware
- Supports both CSV and JSON
    - CSV
        - With or without header, and data shift is more likely to occur
        - settable delimiter
- Supports missing or not provided values

## Overview
- [Mawu](#mawu)
    - [Features](#features)
    - [Overview](#overview)
    - [Naming the Creation: A Legacy of the Divine](#naming-the-creation-a-legacy-of-the-divine)
    - [`MawuValue`](#mawuvalue)
        - [An exhaustive list of all `MawuValue`'s](#an-exhaustive-list-of-all-mawuvalue's)
    - [CSV](#csv)
        - [Handling missing or not provided values](#handling-missing-or-not-provided-values)
            - [With header](#with-header)
            - [Without header](#without-header)
        - [CSV Return value](#csv-return-value)
        - [CSV Usage](#csv-usage)
    - [JSON](#json)

## Naming the creation: A Legacy of the Divine
The name "Mawu" isn't chosen by chance, it honors the powerful West African goddess associated with the moon, the sun, and creation itself.
There's a long and rich human tradition of naming significant things after deities. Mawu embodies this tradition perfectly.

Just as Mawu, the goddess, is linked to creation, Mawu, the library, empowers you to create new things from raw data.  JSON and CSV files are like raw materials, and Mawu provides the tools to shape them into meaningful structures, ready to be used for analysis, manipulation, and ultimately, new creations.

## `MawuValue`
Mawu uses the `MawuValue` enum to represent the different types of values that can be found in both JSON and CSV files, in one, the other, or exclusively.

Both the CSV parser and the JSON parser use a different subset of this enum to represent the different types of values.
The difference is slight however, as only the `array` and `object` are different at all, and are represented as `MawuValue::CsvArray` and `MawuValue::CsvObject` for the CSV parser, and `Mawu::Array` and `Mawu::Object` for the JSON parser.

### An exhaustive list of all `MawuValue`'s
- General types
    - `MawuValue::Null`
    - `MawuValue::Bool`
    - `MawuValue::Int`
    - `MawuValue::Float`
    - `MawuValue::String`
- JSON exclusive types
    - `MawuValue::Array`
    - `MawuValue::Object`
- CSV exclusive types
    - `MawuValue::CsvArray`
    - `MawuValue::CsvObject`

Again, convenience functions for all types are provided by Mawu, in the form of `is_{MawuValue}` and `as_{MawuValue}` functions.
When you call any `as_` function on a `MawuValue` you are returned a `Option()` wrapping the desired value, or `None` if the value is not the type requested. 
Calling `as_null` will return `None` instead when the value is none, and `Some()` otherwise.

#### Example of getting a `MawuValue` if its type is not known or different in the same field

```rust

match mawu_value {
    // General types
    MawuValue::Null => None,
    MawuValue::Bool(b) => b.as_bool(),
    MawuValue::Int(i) => i.as_i64(),
    MawuValue::Float(f) => f.as_f64(),
    MawuValue::String(s) => s.as_str(),
    // Json exclusive types
    MawuValue::Array(a) => a.as_array(),
    MawuValue::Object(o) => o.as_object(),
    // Csv exclusive types
    MawuValue::CsvArray(ca) => ca.as_csv_array(),
    MawuValue::CsvObject(co) => co.as_csv_object(),
}

```

## CSV
This library supports CSV files, conforming to the rfc4180 standard and is itself conforming to the rfc4180 standard and nothing else.

Please note that CSV, while a standard exists, is seldom implemented as such in practice, and almost every implementation of CSV is not conforming to the rfc4180 standard in some way and thus more or less compatible with each other.

One example would be a common shorthand for an array by using `aaa / bbb / ccc` to represent `[aaa, bbb, ccc]`. 
This is not part of the rfc4180 standard and thus not implemented in Mawu, instead it would be treated as a single string, with the appropriate errors.
`aaa / "bbb" / ccc` would produce an error for example, as Mawu treats the entire thing as one string, but it encounters unescaped double-quotes.

Another example is the way encoding is implemented. Mawu uses `utf-8` encoding exclusively for CSV, and does not recognize or produce a `BOM` or similar at the beginning of the file.
There are CSV files encoded in `utf-16`, `utf-32` or even some `ASCII`-variants, and there are some more esoteric implementations like the IBM one where you can define new field names in the middle of a CSV file by using `#GROUP_OBJECT_PROFILE#` [learn more](https://www.ibm.com/docs/en/sig-and-i/10.0.2?topic=schedules-example-comma-separated-value-csv-file).

Because of this, most if not all CSV files are only supported in the ecosystem or app they were created in, and there is no guarantee that Mawu will be able to parse them correctly.

Mawu handles CSV files with an empty or filled last row.

> [!NOTE]
> While the usage of the header is optional, you will need to use either the `read_csv_headless(path)`, or the `read_csv_headed(path)` method.
> [Learn more.](#usage)

### Handling missing or not provided values

The rfc4180 standard allows for missing or not provided values in CSV files only implicitly. There are many different ways libraries have implemented this in the past, and Mawu goes with the closest interpretation the rfc4180 allows.
So while Mawu does handle missing or not provided values, it is, and cannot ever be, 100% reliable.
Exactly how this is handled is explained in the following paragraphs.

Because of the rfc4180 standard, a missing value in the form of `aaa, ,ccc` would still result in 3 `MawuValue`'s in the form of `[aaa][ ][ccc]` as CSV has significant white space, so the missing `bbb` is converted into a space.
A row in the form of `aaa,,ccc` would result in a `MawuValue` of `[aaa][Mawu::None][ccc]` for the same reasons.
One last example is the handling of a value of `""` in the middle of a CSV file. This is also part of the rfc4180 standard only implicitly, and sometimes interpreted as an empty string, other times as a missing value.
Mawu will treat it as an empty string and uses it as the default for any empty value itself.

This library implements missing or not provided values differently depending on if a header is present or not.

#### With header
If a header is present, the missing values will be filled with a `Mawu::None` Value.

A header of `AAA,BBB,CCC`, and the row `aaa,bbb,` would result in a `MawuValue` of `[aaa][bbb][MMawu::None]`.
With a header of `AAA,BBB,CCC,DDD`, the row `aaa,bbb,` would result in a `MawuValue` of `[aaa][bbb][Mawu::None][Mawu::None]`.

Please note that as long as a header is present Mawu will append `Mawu::None` values for as many columns as there are columns declared in the header.


#### Without header
Should a header be not present, any row ending in a `,` will append as many `Mawu::None` values as there are columns in the first row.

The row `aaa,bbb,` would result in a `MawuValue` of `[aaa][bbb][Mawu::None]` because of the trailing comma without content.
A row where the missing value is `aaa,bbb` would result in a `MawuValue` of `[aaa][bbb]` only in the case where it is in the first row.
However, the same row of `aaa,bbb` would result in a `MawuValue` of `[aaa][bbb][Mawu::None]` in the case where the first row is `aaa,bbb,ccc`, or as many `Mawu::None` values as there are columns in the first row.

### CSV Return value
Mawu will return a `Result<MawuValue, MawuError>`. The wrapped `MawuValue` will have one of two types, depending on if a file with a header is parsed or not.

If `Mawu::from_csv_headed(path)` is used, the `MawuValue` will be of type `Vec<Vec<MawuValue>>`, and if `Mawu::from_csv_headless(path)` is used, the `MawuValue` will be of type `Vec<HashMap<String, MawuValue>>`.

To get to your data, you will need to iterate over the contents of the `MawuValue` returned. You can do this by calling `as_csv_object()` or `as_csv_array()` on the `MawuValue` returned as appropriate. If you are not sure what the returned value type is, you can check by using `is_csv_object()` or `is_csv_array()`, convenience functions for all types are provided by Mawu.  

### CSV Usage
Reading a CSV file and just printing out the values:

```rust
use mawu::Mawu::*;

fn main() {
    // for a csv file with header
    let mawu: Vec<HashMap<String, MawuValue>> = Mawu::read_csv_headed("/path/to/file.csv");

    // mawu will return a Result<MawuResult, MawuError>
    for entry in mawu.unwrap().as_csv_object().unwrap() {
        for (key, value) in &entry {
            println!("{}: {}", key, value);
        }
    }

    // for a csv file without header
    let mawu_headless: Vec<Vec<MawuValue>> = Mawu::read_csv_headless("/path/to/file.csv");

    // mawu will return a Result<MawuResult, MawuError>
    for entry in mawu_headless.unwrap().as_csv_array().unwrap() {
        for value in entry {
            println!("{}", value);
        }
    }

}
```

## JSON
This library supports JSON files that conform to the rfc8259 and the ECMA-404 standard.
JSON is one of the most used and common file formats used for data interchange. Defined in 2001 by Douglas Crockford, JSON has gone through several editions and has been used in production for over 20 years.
Because of the several editions and conciseness of JSON grammar, many aspects are left undefined and the various implementations are not consistent in the way they parse JSON.

Mawu is designed to stick as close to the standards as possible, and does not support any common JSON extension like trailing commas.
Most edge cases and the way they are handled are explained in the following paragraphs.

### Edge cases

#### Objects

In the rfc8259 standard, a JSON object is a set of key-value pairs where the keys should be unique. As this is not a hard requirement however, JSON parsers have handled this in a number of ways.
Mawu will parse JSON objects as a `HashMap<String, MawuValue>` and uses the same behavior for duplicate keys, in that they are replaced with the last value.

As the order of object members differs between parsers, Mawu does not order the members in any way, and will use the order in the JSON file.

#### Numbers

`Infinity` and `NaN` are explicitly not part of the rfc8259 standard, but are implemented in some parsers. Mawu does not support them at all.

The rfc8259 doesn't set any limits on the range and precision of numbers, but recommends the implementation of `IEEE 754 binary64`, so Mawu supports any rust `f64` value.
