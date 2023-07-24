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
    pub fn get_head_position(&self) -> Vector3 {
        let address = self.base + 0x4;
        let pos = unsafe {
            let pointer = address as *mut Vector3;
            *pointer
        };

        pos
    }

    pub fn get_body_position(&self) -> Vector3 {
        let address = self.base + 0x28;
        let pos = unsafe {
            let pointer = address as *mut Vector3;
            *pointer
        };

        pos
    }

    pub fn get_angle(&self) -> Vector3 {
        let address = self.base + 0x34;
        let pos = unsafe {
            let pointer = address as *mut Vector3;
            *pointer
        };

        pos
    }
}

impl LuaUserData for Player {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("base", |_, this| Ok(this.base));

        fields.add_field_method_get("health", |_, this| Ok(this.get_health()));
        fields.add_field_method_get("armor", |_, this| Ok(this.get_armor()));
        fields.add_field_method_get("position", |_, this| Ok(this.get_head_position()));
        fields.add_field_method_get("body_position", |_, this| Ok(this.get_body_position()));
        fields.add_field_method_get("angle", |_, this| Ok(this.get_angle()));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(_methods: &mut M) {}
}
