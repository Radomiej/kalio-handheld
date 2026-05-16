use anyhow::Result;
use mlua::{Lua, Result as LuaResult};
use tracing::info;

pub struct ScriptEngine {
    lua: Lua,
}

impl ScriptEngine {
    pub fn new() -> Result<Self> {
        let lua = Lua::new();
        Ok(Self { lua })
    }

    /// Register built-in Kalio API into the Lua global environment.
    pub fn register_api(&self) -> LuaResult<()> {
        let globals = self.lua.globals();

        let log_fn = self.lua.create_function(|_, msg: String| {
            info!(target: "lua", "{}", msg);
            Ok(())
        })?;
        globals.set("log", log_fn)?;

        let print_fn = self.lua.create_function(|_, msg: String| {
            println!("[Lua] {msg}");
            Ok(())
        })?;
        globals.set("print", print_fn)?;

        Ok(())
    }

    pub fn exec_file(&self, path: &str) -> LuaResult<()> {
        let code = std::fs::read_to_string(path)
            .map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;
        self.lua.load(&code).exec()
    }

    pub fn exec_str(&self, code: &str) -> LuaResult<()> {
        self.lua.load(code).exec()
    }
}
