// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_buf
{
	pub buf: *mut c_void,
	pub length: usize,
	pub base: c_int,
	pub hmem: *mut mlx5_hugetlb_mem,
	pub peer: mlx5_peer_direct_mem,
	pub type_: mlx5_alloc_type,
	pub numa_req: mlx5_numa_req,
	pub numa_alloc: c_int,
}

impl Default for mlx5_buf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_buf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_buf {{ buf: {:?}, hmem: {:?}, peer: {:?}, type: {:?}, numa_req: {:?} }}", self.buf, self.hmem, self.peer, self.type_, self.numa_req)
	}
}
