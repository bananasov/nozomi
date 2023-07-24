use log::info;
use mlua::prelude::*;

use crate::nozomi::structs::vector::Vector3;

use super::sdk;
use super::util::pretty_format_multi_value;

pub fn setup_global_env(lua: &Lua) -> mlua::Result<()> {
    let globals = lua.globals();
    globals.set("print", lua.create_function(print)?)?;

    let sdk_table = sdk::setup_sdk_table(&lua)?;
    globals.set("nozomi", sdk_table)?;

    let vector3_constructor = lua.create_function(|_, (x, y, z)| Ok(Vector3::new(x, y, z)))?;
    globals.set("Vector3", vector3_constructor)?;

    Ok(())
}

fn print(_: &Lua, args: LuaMultiValue) -> mlua::Result<()> {
    let formatted = format!("{}", pretty_format_multi_value(&args)?);
    info!("{}", formatted);

    Ok(())
}
