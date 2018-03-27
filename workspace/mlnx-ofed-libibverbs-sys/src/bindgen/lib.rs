// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.



extern crate libc;


use ::std::clone::Clone;
use ::std::default::Default;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::fmt::Result;
use ::std::marker::Copy;
use ::std::marker::PhantomData;
use ::std::mem::transmute;
use ::std::mem::zeroed;
use ::std::option::Option;
use ::std::ops::BitOr;
use ::std::ops::BitOrAssign;
use ::std::ops::BitAnd;
use ::std::ops::BitAndAssign;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::cmp::PartialEq;
use ::std::cmp::Eq;
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_uint;
use ::libc::c_void;

use ::libc::__s32;
use ::libc::__u8;
use ::libc::__u16;
use ::libc::__u32;
use ::libc::__u64;
use ::libc::pthread_mutex_t;
use ::libc::pthread_cond_t;
#[link(name = "ibverbs", kind = "static-nobundle")]
extern "C"
{
}

include!("constants.rs");
include!("functions.rs");
include!("structs.rs");
include!("types.rs");
include!("unions.rs");
