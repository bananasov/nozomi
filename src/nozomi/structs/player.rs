use mlua::prelude::*;

use super::vector::Vector3;

#[derive(Debug)]
pub struct Player {
    pub base: usize,
}

impl Player {
    pub fn get_local_player() -> Player {
        let player1_ptr = 0x400000 + 0x18AC00;
        let base = unsafe {
            let pointer = player1_ptr as *mut usize;
            *pointer
        };

        Player { base }
    }

    // TODO: Add function for getting all players

    pub fn get_health(&self) -> i32 {
        let address = self.base + 0xEC;
        let health = unsafe {
            let pointer = address as *mut i32;
            *pointer
        };

        health
    }

    pub fn get_armor(&self) -> i32 {
        let address = self.base + 0xF0;
        let armor = unsafe {
            let pointer = address as *mut i32;
            *pointer
        };

        armor
    }

    // TODO: Implement getting the 12 byte value named position and cast it into a struct lmao
    pub fn get_position(&self) -> Vector3 {
        Vector3::new(0f32, 0f32, 0f32)
    }
}

impl LuaUserData for Player {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("base", |_, this| Ok(this.base));

        fields.add_field_method_get("health", |_, this| Ok(this.get_health()));
        fields.add_field_method_get("armor", |_, this| Ok(this.get_armor()));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
