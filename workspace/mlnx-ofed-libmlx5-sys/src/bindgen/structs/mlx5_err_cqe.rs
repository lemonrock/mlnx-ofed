// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_err_cqe
{
	pub rsvd0: [u8; 32usize],
	pub srqn: u32,
	pub rsvd1: [u8; 16usize],
	pub hw_err_synd: u8,
	pub hw_synd_type: u8,
	pub vendor_err_synd: u8,
	pub syndrome: u8,
	pub s_wqe_opcode_qpn: u32,
	pub wqe_counter: u16,
	pub signature: u8,
	pub op_own: u8,
}

impl Default for mlx5_err_cqe
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_err_cqe
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"mlx5_err_cqe {{ rsvd0: [{}], rsvd1: {:?} }}",
			self.rsvd0
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
				.collect::<String>(),
			self.rsvd1
		)
	}
}
