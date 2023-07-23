use mlua::prelude::*;

use std::fmt::Write;

// Thanks, lune!
pub fn pretty_format_multi_value(multi: &LuaMultiValue) -> LuaResult<String> {
    let mut buffer = String::new();
    let mut counter = 0;
    for value in multi {
        counter += 1;
        if let LuaValue::String(s) = value {
            write!(buffer, "{}", s.to_string_lossy()).map_err(LuaError::external)?;
        } else {
            // pretty_format_value(&mut buffer, value, 0).map_err(LuaError::external)?;
            write!(buffer, "{:?}", value).unwrap(); // guh
        }
        if counter < multi.len() {
            write!(&mut buffer, " ").map_err(LuaError::external)?;
        }
    }
    Ok(buffer)
}
