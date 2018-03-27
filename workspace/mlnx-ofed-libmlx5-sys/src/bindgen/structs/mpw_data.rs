// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mpw_data
{
	pub state: u8,
	pub size: u8,
	pub num_sge: u8,
	pub len: u32,
	pub total_len: u32,
	pub flags: u32,
	pub scur_post: u32,
	pub __bindgen_anon_1: mpw_data__bindgen_ty_1,
	pub ctrl_update: *mut u32,
}

impl Default for mpw_data
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mpw_data
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mpw_data {{  }}")
	}
}
