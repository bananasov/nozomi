use log::info;
use mlua::prelude::*;

use super::sdk;
use super::util::pretty_format_multi_value;

pub fn setup_global_env(lua: &Lua) -> mlua::Result<()> {
    let globals = lua.globals();
    globals.set("print", lua.create_function(print)?)?;

    let sdk_table = sdk::setup_sdk_table(&lua)?;
    globals.set("sdk", sdk_table)?;

    Ok(())
}

fn print(_: &Lua, args: LuaMultiValue) -> mlua::Result<()> {
    let formatted = format!("{}", pretty_format_multi_value(&args)?);
    info!("{}", formatted);

    Ok(())
}
