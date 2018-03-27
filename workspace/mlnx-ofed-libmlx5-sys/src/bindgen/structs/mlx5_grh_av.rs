// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_grh_av
{
	pub reserved0: [u8; 4usize],
	pub rmac: [u8; 6usize],
	pub tclass: u8,
	pub hop_limit: u8,
	pub grh_gid_fl: u32,
	pub rgid: [u8; 16usize],
}

impl Default for mlx5_grh_av
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_grh_av
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_grh_av {{ reserved0: {:?}, rmac: {:?}, rgid: {:?} }}", self.reserved0, self.rmac, self.rgid)
	}
}
