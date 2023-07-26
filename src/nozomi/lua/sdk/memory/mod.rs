use mlua::prelude::*;

macro_rules! make_unsafe_mem_read_block {
    ($address:expr, $type:ty) => {
        unsafe {
            let pointer = $address as *mut $type;
            *pointer
        }
    };
}

macro_rules! make_unsafe_mem_write_block {
    ($address:expr, $type:ty, $value:expr) => {
        unsafe {
            let pointer = $address as *mut $type;
            *pointer = $value
        }
    };
}

macro_rules! unpack_lua_value {
    ($lua:expr, $args:expr, $index:expr, $type:ty) => {{
        let lua_value = $args.get($index).unwrap();
        let argument: $type = $lua.unpack(lua_value.clone())?;
        argument
    }};
}

pub fn read_u8(_: &Lua, address: usize) -> mlua::Result<u8> {
    let value = make_unsafe_mem_read_block!(address, u8);

    Ok(value)
}

pub fn read_u16(_: &Lua, address: usize) -> mlua::Result<u16> {
    let value = make_unsafe_mem_read_block!(address, u16);

    Ok(value)
}

pub fn read_u32(_: &Lua, address: usize) -> mlua::Result<u32> {
    let value = make_unsafe_mem_read_block!(address, u32);

    Ok(value)
}

pub fn read_u64(_: &Lua, address: usize) -> mlua::Result<u64> {
    let value = make_unsafe_mem_read_block!(address, u64);

    Ok(value)
}

pub fn read_f32(_: &Lua, address: usize) -> mlua::Result<f32> {
    let value = make_unsafe_mem_read_block!(address, f32);

    Ok(value)
}

pub fn read_f64(_: &Lua, address: usize) -> mlua::Result<f64> {
    let value = make_unsafe_mem_read_block!(address, f64);

    Ok(value)
}

pub fn read_usize(_: &Lua, address: usize) -> mlua::Result<usize> {
    let value: usize = make_unsafe_mem_read_block!(address, usize);

    Ok(value)
}

pub fn write_u8(lua: &Lua, args: LuaMultiValue) -> mlua::Result<u8> {
    let address = unpack_lua_value!(lua, args, 0, usize);
    let value = unpack_lua_value!(lua, args, 1, u8);

    make_unsafe_mem_write_block!(address, u8, value);

    Ok(value)
}

pub fn write_u16(lua: &Lua, args: LuaMultiValue) -> mlua::Result<u16> {
    let address = unpack_lua_value!(lua, args, 0, usize);
    let value = unpack_lua_value!(lua, args, 1, u16);

    make_unsafe_mem_write_block!(address, u16, value);

    Ok(value)
}

pub fn write_u32(lua: &Lua, args: LuaMultiValue) -> mlua::Result<u32> {
    let address = unpack_lua_value!(lua, args, 0, usize);
    let value = unpack_lua_value!(lua, args, 1, u32);

    make_unsafe_mem_write_block!(address, u32, value);

    Ok(value)
}

pub fn write_u64(lua: &Lua, args: LuaMultiValue) -> mlua::Result<u64> {
    let address = unpack_lua_value!(lua, args, 0, usize);
    let value = unpack_lua_value!(lua, args, 1, u64);

    make_unsafe_mem_write_block!(address, u64, value);

    Ok(value)
}

pub fn write_usize(lua: &Lua, args: LuaMultiValue) -> mlua::Result<usize> {
    let address = unpack_lua_value!(lua, args, 0, usize);
    let value = unpack_lua_value!(lua, args, 1, usize);

    make_unsafe_mem_write_block!(address, usize, value);

    Ok(value)
}

pub fn write_f32(lua: &Lua, args: LuaMultiValue) -> mlua::Result<f32> {
    let address = unpack_lua_value!(lua, args, 0, usize);
    let value = unpack_lua_value!(lua, args, 1, f32);

    let _ = make_unsafe_mem_write_block!(address, f32, value);

    Ok(value)
}

pub fn write_f64(lua: &Lua, args: LuaMultiValue) -> mlua::Result<f64> {
    let address = unpack_lua_value!(lua, args, 0, usize);
    let value = unpack_lua_value!(lua, args, 1, f64);

    let _ = make_unsafe_mem_write_block!(address, f64, value);

    Ok(value)
}
