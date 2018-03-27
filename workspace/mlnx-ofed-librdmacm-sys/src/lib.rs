// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(static_nobundle)]
#![feature(untagged_unions)]


#[cfg(target_os = "linux")] extern crate mlnx_ofed_libibverbs_sys;


pub type __be16 = __u16;
pub type __be32 = __u32;
pub type __be64 = __u64;


use ::mlnx_ofed_libibverbs_sys::*;


#[cfg(target_os = "linux")] include!("bindgen/lib.rs");
