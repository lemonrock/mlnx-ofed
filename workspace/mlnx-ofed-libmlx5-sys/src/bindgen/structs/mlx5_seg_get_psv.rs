// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_seg_get_psv
{
	pub rsvd: [u8; 19usize],
	pub num_psv: u8,
	pub l_key: u32,
	pub va: u64,
	pub psv_index: [u32; 4usize],
}

impl Default for mlx5_seg_get_psv
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_seg_get_psv
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_seg_get_psv {{ rsvd: {:?}, psv_index: {:?} }}", self.rsvd, self.psv_index)
	}
}
