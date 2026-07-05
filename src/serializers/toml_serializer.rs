use crate::{errors::toml_error::TomlWriteError, utils::make_whitespace};
use athena::{Array, Number, Object, Table, XffString, XffValue};
use nemesis::NemesisError;

pub fn serialize_toml(value: XffValue, spaces: u8, depth: u16) -> Result<String, NemesisError> {
    let mut out: String = Default::default();
    let value = match value {
        XffValue::Object(obj) => obj,
        _ => {
            return Err(NemesisError::new(
                "mawu::serializers::toml_serializer::serialize_toml",
                TomlWriteError::ParentMustBeObject,
            ));
        }
    };
    for (key, value) in value.iter() {
        serialize_value(key, value, spaces, depth, &mut out)?;
    }
    Ok(out)
}

fn serialize_value(
    key: &str,
    value: &XffValue,
    spaces: u8,
    depth: u16,
    out: &mut String,
) -> Result<(), NemesisError> {
    let next_depth = depth.saturating_add(1);
    let current_whitespace = (spaces as usize).saturating_mul(depth as usize);
    let next_whitespace = (spaces as usize).saturating_mul(next_depth as usize);
    match value {
        XffValue::Graph(_)
        | XffValue::Data(_)
        | XffValue::CommandCharacter(_)
        | XffValue::ArrayCmdChar(_) => {
            return Err(NemesisError::new(
            "mawu::serializers::toml_serializer::serialize_value",
            TomlWriteError::NotTOMLType(
                "A `XffValue::Graph` or `XffValue::Data` cannot be serialized to TOML".to_string(),
            ),
        ).add_ctx("Should you want to store binary data, please consider using `nabu` and working with `.xff` files directly. It is never advisable to store binary data in a file for human interaction.").add_ctx("If you want to store a graph, please also consider using `nabu`."));
        }
        XffValue::Null => {
            out.push_str(&format!(
                "{}{} = null\n",
                make_whitespace(current_whitespace),
                key
            ));
        }
        XffValue::NaN => {
            out.push_str(&format!(
                "{}{} = nan\n",
                make_whitespace(current_whitespace),
                key
            ));
        }
        XffValue::NegNaN => {
            out.push_str(&format!(
                "{}{} = -nan\n",
                make_whitespace(current_whitespace),
                key
            ));
        }
        XffValue::PosNaN => {
            out.push_str(&format!(
                "{}{} = +nan\n",
                make_whitespace(current_whitespace),
                key
            ));
        }
        XffValue::Infinity => {
            out.push_str(&format!(
                "{}{} = inf\n",
                make_whitespace(current_whitespace),
                key
            ));
        }
        XffValue::NegInfinity => {
            out.push_str(&format!(
                "{}{} = -inf\n",
                make_whitespace(current_whitespace),
                key
            ));
        }
        XffValue::Uuid(uuid) => {
            out.push_str(&format!(
                "{}{} = {}\n",
                make_whitespace(current_whitespace),
                key,
                uuid
            ));
        }
        XffValue::DateTime(dt) => {
            out.push_str(&format!(
                "{}{} = {}\n",
                make_whitespace(current_whitespace),
                key,
                dt
            ));
        }
        XffValue::LocalDateTime(ldt) => {
            out.push_str(&format!(
                "{}{} = {}\n",
                make_whitespace(current_whitespace),
                key,
                ldt
            ));
        }
        XffValue::LocalTime(lt) => {
            out.push_str(&format!(
                "{}{} = {}\n",
                make_whitespace(current_whitespace),
                key,
                lt
            ));
        }
        XffValue::LocalDate(ld) => {
            out.push_str(&format!(
                "{}{} = {}\n",
                make_whitespace(current_whitespace),
                key,
                ld
            ));
        }
        XffValue::Duration(d) => {
            out.push_str(&format!(
                "{}{} = {}\n",
                make_whitespace(current_whitespace),
                key,
                d
            ));
        }
        XffValue::Ascii(str) | XffValue::String(str) => {
            serialize_string_to_toml(str, key, current_whitespace, out);
        }
        XffValue::Table(table) => {
            serialize_table_to_toml(table, key, current_whitespace, depth, out)?;
        }
        XffValue::Object(obj) => {
            serialize_object_to_toml(obj, key, current_whitespace, out);
        }
        XffValue::Array(arr) => {
            serialize_array_to_toml(arr, key, current_whitespace, out);
        }
        XffValue::Number(n) => {
            serialize_number_to_toml(n, key, current_whitespace, out);
        }
        XffValue::HpFloat(hpf) => {
            let v = hpf.get_value();
            let s = hpf.get_scale();
            // TODO: Make this more accurate
            let num = v as f64 * 10f64.powf(s as f64);
            out.push_str(&format!(
                "{}{} = {}\n",
                make_whitespace(current_whitespace),
                key,
                num
            ));
        }
        XffValue::Metadata(m) => {
            serialize_object_to_toml(m.as_object(), key, current_whitespace, out);
        }
        XffValue::OrderedObject(ord) => {
            let mut obj = Object::new();
            for (k, v) in ord.iter() {
                obj.insert(k.clone(), v.clone());
            }
            serialize_object_to_toml(&obj, key, current_whitespace, out);
        }
        XffValue::Boolean(b) => {
            out.push_str(&format!(
                "{}{} = {}\n",
                make_whitespace(current_whitespace),
                key,
                b
            ));
        }
    }
    Ok(())
}

fn serialize_string_to_toml(
    str: &XffString,
    key: &str,
    current_whitespace: usize,
    out: &mut String,
) {
    out.push_str(&format!(
        "{}{} = \"{}\"\n",
        make_whitespace(current_whitespace),
        key,
        str
    ));
}

fn serialize_table_to_toml(
    table: &Table,
    key: &str,
    current_whitespace: usize,
    depth: u16,
    out: &mut String,
) -> Result<(), NemesisError> {
    let mut index = 0;
    while let Some(row) = table.get_row(index) {
        index = index.saturating_add(1);
        out.push_str(&format!("[[{}]]\n", key));
        let ord_obj = row
            .as_ordered_object()
            .expect("Row is always returned as an OrderedObject");
        for (k, v) in ord_obj.iter() {
            serialize_value(k, v, current_whitespace as u8, depth + 1, out)?;
        }
    }
    Ok(())
}

fn serialize_array_to_toml(arr: &Array, key: &str, current_whitespace: usize, out: &mut String) {
    out.push_str(&format!(
        "{}{} = [",
        make_whitespace(current_whitespace),
        key
    ));
    for (i, v) in arr.iter().enumerate() {
        if out.ends_with('\n') {
            out.push_str(&make_whitespace(current_whitespace));
        }
        let v = v.to_string();
        if i == 0 && v.len() < 50 {
            out.push_str("\n");
            out.push_str(&make_whitespace(current_whitespace));
        }
        out.push_str(&v);
        if i < arr.len() - 1 {
            if v.len() < 50 {
                out.push_str(", ");
            } else {
                out.push_str(",\n");
            }
        }
    }
    out.push_str("]\n");
}

fn serialize_number_to_toml(num: &Number, key: &str, current_whitespace: usize, out: &mut String) {
    match num {
        Number::Unsigned(u) => {
            out.push_str(&format!(
                "{}{} = {}\n",
                make_whitespace(current_whitespace),
                key,
                u
            ));
        }
        Number::Integer(i) => {
            out.push_str(&format!(
                "{}{} = {}\n",
                make_whitespace(current_whitespace),
                key,
                i
            ));
        }
        Number::Float(f) => {
            out.push_str(&format!(
                "{}{} = {}\n",
                make_whitespace(current_whitespace),
                key,
                f
            ));
        }
    }
}

fn serialize_object_to_toml(obj: &Object, key: &str, current_whitespace: usize, out: &mut String) {
    out.push_str(&format!("[{}]\n", key));
    for (k, v) in obj.iter() {
        serialize_value(k, v, current_whitespace as u8, 1, out).unwrap();
    }
}
