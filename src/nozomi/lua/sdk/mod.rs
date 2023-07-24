use mlua::prelude::*;

use crate::{nozomi::structs::player::Player, MODULE};

mod memory;

pub fn setup_sdk_table(lua: &Lua) -> mlua::Result<LuaTable> {
    let sdk_table = lua.create_table()?; // ill care about error handling later.

    let memory_table = lua.create_table()?;

    memory_table.set(
        "get_base_address",
        lua.create_function(|_, ()| {
            let proc = MODULE.read().unwrap();
            let base_address = proc.as_ref().unwrap().base_address;

            Ok(base_address)
        })?,
    )?;

    memory_table.set("read_u8", lua.create_function(memory::read_u8)?)?;
    memory_table.set("read_u16", lua.create_function(memory::read_u16)?)?;
    memory_table.set("read_u32", lua.create_function(memory::read_u32)?)?;
    memory_table.set("read_u64", lua.create_function(memory::read_u64)?)?;
    memory_table.set("read_usize", lua.create_function(memory::read_usize)?)?;

    memory_table.set("write_u8", lua.create_function(memory::write_u8)?)?;
    memory_table.set("write_u16", lua.create_function(memory::write_u16)?)?;
    memory_table.set("write_u32", lua.create_function(memory::write_u32)?)?;
    memory_table.set("write_u64", lua.create_function(memory::write_u64)?)?;
    memory_table.set("write_usize", lua.create_function(memory::write_usize)?)?;

    memory_table.set("read_f64", lua.create_function(memory::read_f64)?)?;
    memory_table.set("read_f32", lua.create_function(memory::read_f32)?)?;

    memory_table.set("write_f64", lua.create_function(memory::write_f64)?)?;
    memory_table.set("write_f32", lua.create_function(memory::write_f32)?)?;

    // Player stuff
    sdk_table.set(
        "get_local_player",
        lua.create_function(|_lua: &Lua, ()| {
            let localplayer = Player::get_local_player();

            Ok(localplayer)
        })?,
    )?;
    sdk_table.set("get_players", "TODO")?;

    // Memory editing from Lua
    sdk_table.set("memory", memory_table)?;

    Ok(sdk_table)
}
