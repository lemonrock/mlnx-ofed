// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ucma_abi_query_path_resp
{
	pub num_paths: __u32,
	pub reserved: __u32,
	pub path_data: __IncompleteArrayField<ibv_path_data>,
}

impl Default for ucma_abi_query_path_resp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ucma_abi_query_path_resp
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ucma_abi_query_path_resp {{ path_data: {:?} }}", self.path_data)
	}
}
