// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ib_exp_modify_wq
{
	pub hdr: ex_hdr,
	pub comp_mask: __u32,
	pub wq_handle: __u32,
	pub wq_state: __u32,
	pub curr_wq_state: __u32,
}

impl Default for ib_exp_modify_wq
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ib_exp_modify_wq
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ib_exp_modify_wq {{ hdr: {:?} }}", self.hdr)
	}
}
