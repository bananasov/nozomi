use mlua::prelude::*;

use super::vector::Vector3;
use crate::{nozomi::offsets::entity::{
    ANGLE, ARMOR as PLAYER_ARMOR, BODY_POSITION, HEAD_POSITION, HEALTH as PLAYER_HEALTH,
    LOCAL_PLAYER_ADDRESS,
}, MODULE};

#[derive(Debug)]
pub struct Player {
    pub base: usize,
}

impl Player {
    pub fn get_local_player() -> Player {
        let player1_ptr = 0x400000 + LOCAL_PLAYER_ADDRESS;
        let base = unsafe {
            let pointer = player1_ptr as *mut usize;
            *pointer
        };

        Player { base }
    }

    // TODO: Add function for getting all players

    pub fn get_health(&self) -> i32 {
        let address = self.base + PLAYER_HEALTH;
        let health = unsafe {
            let pointer = address as *mut i32;
            *pointer
        };

        health
    }

    pub fn get_armor(&self) -> i32 {
        let address = self.base + PLAYER_ARMOR;
        let armor = unsafe {
            let pointer = address as *mut i32;
            *pointer
        };

        armor
    }

    // TODO: Implement getting the 12 byte value named position and cast it into a struct lmao
    pub fn get_head_position(&self) -> Vector3 {
        let address = self.base + HEAD_POSITION;
        let pos = unsafe {
            let pointer = address as *mut Vector3;
            *pointer
        };

        pos
    }

    pub fn get_body_position(&self) -> Vector3 {
        let address = self.base + BODY_POSITION;
        let pos = unsafe {
            let pointer = address as *mut Vector3;
            *pointer
        };

        pos
    }

    pub fn get_angle(&self) -> Vector3 {
        let address = self.base + ANGLE;
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
