// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#![feature(static_nobundle)]


#[cfg(target_os = "linux")] extern crate libnuma_sys;
#[cfg(target_os = "linux")] extern crate mlnx_ofed_libibverbs_sys;


#[cfg(target_os = "linux")] include!("bindgen/lib.rs");
