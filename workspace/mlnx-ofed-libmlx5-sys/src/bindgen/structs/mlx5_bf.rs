// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_bf
{
	pub reg: *mut c_void,
	pub need_lock: c_int,
	pub lock: mlx5_lock,
	pub offset: c_uint,
	pub buf_size: c_uint,
	pub uuarn: c_uint,
	pub db_method: mlx5_db_method,
}

impl Default for mlx5_bf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_bf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_bf {{ reg: {:?}, lock: {:?}, db_method: {:?} }}", self.reg, self.lock, self.db_method)
	}
}
