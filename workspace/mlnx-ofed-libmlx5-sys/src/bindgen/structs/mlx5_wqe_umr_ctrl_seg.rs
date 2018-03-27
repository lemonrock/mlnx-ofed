// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_wqe_umr_ctrl_seg
{
	pub flags: u8,
	pub rsvd0: [u8; 3usize],
	pub klm_octowords: u16,
	pub translation_offset: u16,
	pub mkey_mask: u64,
	pub rsvd1: [u8; 32usize],
}

impl Default for mlx5_wqe_umr_ctrl_seg
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_wqe_umr_ctrl_seg
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"mlx5_wqe_umr_ctrl_seg {{ rsvd0: {:?}, rsvd1: [{}] }}",
			self.rsvd0,
			self.rsvd1
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>()
		)
	}
}