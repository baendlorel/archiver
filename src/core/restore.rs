use owo_colors::OwoColorize;

use crate::{err_info, wrap_err_fatal, wrap_result};

use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use super::{list, log};
use crate::{
    misc::{ForceToString, paths},
    models::{
        error::ArchiverError,
        types::{ListEntry, OperType},
    },
};

pub fn handler
