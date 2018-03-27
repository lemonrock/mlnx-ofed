// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_rereg_mr
{
	pub hdr: ex_hdr,
	pub comp_mask: __u32,
	pub mr_handle: __u32,
	pub flags: __u32,
	pub reserved: __u32,
	pub start: __u64,
	pub length: __u64,
	pub hca_va: __u64,
	pub pd_handle: __u32,
	pub access_flags: __u32,
}

impl Default for ibv_exp_rereg_mr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_rereg_mr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_rereg_mr {{ hdr: {:?} }}", self.hdr)
	}
}
