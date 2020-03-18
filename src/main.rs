use rlua::{FromLuaMulti, Lua, Result};

fn eval<R: for<'lua> FromLuaMulti<'lua>>(script: &str) -> Result<R> {
    Lua::new().context(|lua_ctx| lua_ctx.load(script).eval())
}

fn main() -> Result<()> {
    assert_eq!(
        eval::<Vec<String>>("return { \"one\", \"two\" }")?,
        vec!["one", "two"]
    );
    assert_eq!(
        eval::<String>("return \"Hello world\"")?,
        String::from("Hello world")
    );
    eval("print(\"Hello world\")")?;
    Ok(())
}
