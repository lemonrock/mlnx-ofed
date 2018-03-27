// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct general_data_hot
{
	pub wqe_head: *mut c_uint,
	pub post_send_one: Option<unsafe extern "C" fn(wr: *mut ibv_exp_send_wr, qp: *mut mlx5_qp, exp_send_flags: u64, seg: *mut c_void, total_size: *mut c_int) -> c_int>,
	pub sqstart: *mut c_void,
	pub sqend: *mut c_void,
	pub db: *mut u32,
	pub bf: *mut mlx5_bf,
	pub scur_post: u32,
	pub last_post: u32,
	pub create_flags: u16,
	pub fm_cache: u8,
	pub model_flags: u8,
}

impl Default for general_data_hot
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for general_data_hot
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "general_data_hot {{  }}")
	}
}
