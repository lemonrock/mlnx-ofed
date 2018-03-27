// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_wqe_eth_seg
{
	pub rsvd0: u32,
	pub cs_flags: u8,
	pub rsvd1: u8,
	pub mss: u16,
	pub rsvd2: u32,
	pub inline_hdr_sz: u16,
	pub inline_hdr_start: [u8; 2usize],
	pub inline_hdr: [u8; 16usize],
}

impl Default for mlx5_wqe_eth_seg
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_wqe_eth_seg
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_wqe_eth_seg {{ inline_hdr_start: {:?}, inline_hdr: {:?} }}", self.inline_hdr_start, self.inline_hdr)
	}
}
