// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_mlx5_cq_info
{
	pub cqn: u32,
	pub cqe_cnt: c_uint,
	pub buf: *mut c_void,
	pub dbrec: *mut u32,
	pub cqe_size: c_uint,
}

impl Default for ibv_mlx5_cq_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_mlx5_cq_info
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_mlx5_cq_info {{ buf: {:?}, dbrec: {:?} }}", self.buf, self.dbrec)
	}
}
