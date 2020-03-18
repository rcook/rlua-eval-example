use rlua::{Lua, Result};

fn eval0(script: &str) -> Result<Vec<String>> {
    Lua::new().context(|lua_ctx| lua_ctx.load(script).eval())
}

fn eval1(script: &str) -> Result<String> {
    Lua::new().context(|lua_ctx| lua_ctx.load(script).eval())
}

fn eval2(script: &str) -> Result<()> {
    Lua::new().context(|lua_ctx| lua_ctx.load(script).eval())
}

// How do I write a generic `eval` function?
/*
fn eval<R>(script: &str) -> Result<R> {
    Lua::new().context(|lua_ctx| lua_ctx.load(script).eval())
}
*/

fn main() -> Result<()> {
    assert_eq!(eval0("return { \"one\", \"two\" }")?, vec!["one", "two"]);
    assert_eq!(
        eval1("return \"Hello world\"")?,
        String::from("Hello world")
    );
    eval2("print(\"Hello world\")")?;
    Ok(())
}
