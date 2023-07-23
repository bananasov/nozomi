use mlua::prelude::*;

mod global_env;

pub mod sdk;
pub mod util;

pub struct NozomiLua {
    lua: Lua, // All we need really.
}

impl NozomiLua {
    pub fn new() -> Result<NozomiLua, Box<dyn std::error::Error>> {
        let lua = Lua::new();

        global_env::setup_global_env(&lua)?;

        Ok(Self { lua })
    }

    pub fn load(self, source: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.lua.load(source).exec()?;

        Ok(())
    }
}
