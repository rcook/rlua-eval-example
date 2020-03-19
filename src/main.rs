mod lua {
    pub fn eval<T: for<'lua> rlua::FromLuaMulti<'lua>>(script: &str) -> rlua::Result<T> {
        rlua::Lua::new().context(|lua_ctx| lua_ctx.load(script).eval())
    }
}

mod js {
    pub type Result<T> = std::result::Result<T, &'static str>;

    pub trait FromJS: Default {}

    // Default implementation for types that are sized and have defaults
    impl<T: ?Sized + Default> FromJS for T {}

    pub fn eval<T: FromJS>(_script: &str) -> Result<T> {
        Ok(T::default())
    }
}

mod scripting {
    use super::js;
    use super::lua;
    use std::sync::Arc;

    // https://www.reddit.com/r/rust/comments/fkrakp/rlua_how_do_i_make_a_generic_eval_function/
    pub trait Scriptable: for<'lua> rlua::FromLuaMulti<'lua> + js::FromJS {}

    // Default implementation for types that can be converted from Lua and JavaScript
    impl<T: for<'lua> rlua::FromLuaMulti<'lua> + js::FromJS> Scriptable for T {}

    #[derive(Debug, Clone)]
    pub enum Error {
        Lua(Arc<rlua::Error>),
        JavaScript(String),
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Error::Lua(error) => write!(f, "Lua({})", error),
                Error::JavaScript(message) => write!(f, "JavaScript({})", message),
            }
        }
    }

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            None
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;

    pub enum Language {
        Lua,
        JavaScript,
    }

    pub fn eval<T: Scriptable>(language: Language, script: &str) -> Result<T> {
        match language {
            Language::Lua => lua::eval(script).map_err(|error| Error::Lua(Arc::new(error))),
            Language::JavaScript => {
                js::eval(script).map_err(|message| Error::JavaScript(message.to_string()))
            }
        }
    }
}

fn main() -> scripting::Result<()> {
    use scripting::*;

    assert_eq!(
        scripting::eval::<Vec<String>>(Language::Lua, "return { \"one\", \"two\" }")?,
        vec!["one", "two"]
    );
    assert_eq!(
        scripting::eval::<String>(Language::Lua, "return \"Hello world\"")?,
        String::from("Hello world")
    );
    scripting::eval(Language::Lua, "print(\"Hello world\")")?;
    scripting::eval(Language::JavaScript, "print(\"Hello world\")")?;
    Ok(())
}
