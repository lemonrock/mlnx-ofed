// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_peer_direct_mem
{
	pub dir: u32,
	pub va_id: u64,
	pub pb: *mut ibv_exp_peer_buf,
	pub ctx: *mut ibv_exp_peer_direct_attr,
}

impl Default for mlx5_peer_direct_mem
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_peer_direct_mem
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_peer_direct_mem {{ pb: {:?}, ctx: {:?} }}", self.pb, self.ctx)
	}
}
