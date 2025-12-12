#[cfg(feature = "crossplat")]
pub(crate) mod crossplat {
    pub(crate) mod component {
        pub(crate) mod net {
            pub(crate) mod net;
        }
        pub(crate) mod cmd;
        pub(crate) mod ui;
    }
    pub(crate) mod consts;
}

pub use crate::{
    crossplat::{
        component::{
            net::{
                net::*,
            },
            cmd::*,
            ui::*,
        },
    },
};

pub(crate) use crate::{
    crossplat::{
        consts::*,
    },
};

pub(crate) use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
    sync::LazyLock,
};

pub(crate) use sourcetrait_crossplat_bridge::{
    self as cross,
    //prelude::driver::*,
    CommandReturn, CmdKind,
};
pub(crate) use sourcetrait_crossplat_unix as unix;
pub(crate) use sourcetrait_stdx::process::CommandExt;
//pub(crate) use sourcetrait_twostr::*;
