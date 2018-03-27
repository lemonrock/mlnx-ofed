// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_mem_repeat_block
{
	pub base_addr: u64,
	pub mr: *mut ibv_mr,
	pub byte_count: *mut usize,
	pub stride: *mut usize,
}

impl Default for ibv_exp_mem_repeat_block
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_mem_repeat_block
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_mem_repeat_block {{ mr: {:?}, byte_count: {:?}, stride: {:?} }}", self.mr, self.byte_count, self.stride)
	}
}
