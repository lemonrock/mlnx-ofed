// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_query_mkey
{
	pub hdr: ex_hdr,
	pub comp_mask: __u64,
	pub handle: __u32,
	pub lkey: __u32,
	pub rkey: __u32,
	pub reserved: __u32,
	pub driver_data: __IncompleteArrayField<__u64>,
}

impl Default for ibv_exp_query_mkey
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_query_mkey
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_query_mkey {{ hdr: {:?}, driver_data: {:?} }}", self.hdr, self.driver_data)
	}
}
