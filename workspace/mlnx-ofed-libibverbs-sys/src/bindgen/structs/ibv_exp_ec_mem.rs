// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_ec_mem
{
	pub data_blocks: *mut ibv_sge,
	pub num_data_sge: c_int,
	pub code_blocks: *mut ibv_sge,
	pub num_code_sge: c_int,
	pub block_size: c_int,
}

impl Default for ibv_exp_ec_mem
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_ec_mem
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_ec_mem {{ data_blocks: {:?}, code_blocks: {:?} }}", self.data_blocks, self.code_blocks)
	}
}
