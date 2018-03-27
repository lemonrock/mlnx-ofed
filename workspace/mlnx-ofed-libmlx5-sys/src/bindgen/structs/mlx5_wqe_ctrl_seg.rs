// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_wqe_ctrl_seg
{
	pub opmod_idx_opcode: u32,
	pub qpn_ds: u32,
	pub signature: u8,
	pub rsvd: [u8; 2usize],
	pub fm_ce_se: u8,
	pub imm: u32,
}

impl Default for mlx5_wqe_ctrl_seg
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_wqe_ctrl_seg
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_wqe_ctrl_seg {{ rsvd: {:?} }}", self.rsvd)
	}
}
