mod lua {
    pub trait Evaluatable: for<'lua> rlua::FromLuaMulti<'lua> {}
    pub type EvalError = rlua::Error;
    pub type EvalResult<T> = std::result::Result<T, EvalError>;

    // Default implementation for types that have FromLuaMulti implementations
    impl<T: for<'lua> rlua::FromLuaMulti<'lua>> Evaluatable for T {}

    pub fn eval<T: Evaluatable>(script: &str) -> EvalResult<T> {
        rlua::Lua::new().context(|lua_ctx| lua_ctx.load(script).eval())
    }
}

mod js {
    pub trait Evaluatable: Default {}
    pub type EvalError = &'static str;
    pub type EvalResult<T> = std::result::Result<T, EvalError>;

    // Default implementation for types that are sized and have defaults
    impl<T: ?Sized + Default> Evaluatable for T {}

    pub fn eval<T: Evaluatable>(_script: &str) -> EvalResult<T> {
        Ok(T::default())
    }
}

mod scripting {
    use super::js;
    use super::lua;
    use std::sync::Arc;

    // https://www.reddit.com/r/rust/comments/fkrakp/rlua_how_do_i_make_a_generic_eval_function/
    pub trait Evaluatable: lua::Evaluatable + js::Evaluatable {}

    // Default implementation for types that can be converted from Lua and JavaScript
    impl<T: lua::Evaluatable + js::Evaluatable> Evaluatable for T {}

    #[derive(Debug, Clone)]
    pub enum Error {
        Lua(Arc<lua::EvalError>),
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

    impl std::convert::From<lua::EvalError> for Error {
        fn from(error: lua::EvalError) -> Self {
            Error::Lua(Arc::new(error))
        }
    }

    impl std::convert::From<js::EvalError> for Error {
        fn from(error: js::EvalError) -> Self {
            Error::JavaScript(error.to_string())
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;

    pub enum Language {
        Lua,
        JavaScript,
    }

    pub fn eval<T: Evaluatable>(language: Language, script: &str) -> Result<T> {
        match language {
            Language::Lua => Ok(lua::eval(script)?),
            Language::JavaScript => Ok(js::eval(script)?),
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
