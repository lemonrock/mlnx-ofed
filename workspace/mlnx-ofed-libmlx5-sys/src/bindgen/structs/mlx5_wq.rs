// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_wq
{
	pub wrid: *mut u64,
	pub wqe_cnt: c_uint,
	pub head: c_uint,
	pub tail: c_uint,
	pub max_post: c_uint,
	pub max_gs: c_int,
	pub lock: mlx5_lock,
	pub buff: *mut c_void,
	pub db: *mut u32,
	pub wqe_shift: c_int,
	pub offset: c_int,
	pub wr_data: *mut u32,
}

impl Default for mlx5_wq
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_wq
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_wq {{ wrid: {:?}, lock: {:?}, buff: {:?}, db: {:?}, wr_data: {:?} }}", self.wrid, self.lock, self.buff, self.db, self.wr_data)
	}
}
