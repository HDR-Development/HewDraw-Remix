#![feature(const_trait_impl)]

pub mod __private {
    pub use smash;

    pub const fn const_add_index(
        first: super::VariableIndex,
        second: super::VariableIndex,
    ) -> super::VariableIndex {
        super::VariableIndex {
            flag: first.flag + second.flag,
            word: first.word + second.word,
            dword: first.dword + second.dword,
            float: first.float + second.float,
        }
    }
}

mod module;
mod variable;

pub use variable::*;
pub use variables_macros::agent_variables;

pub use self::module::{add_var_amount as add, VarModule};
