// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_rwq_ind_table
{
	pub context: *mut ibv_context,
	pub pd: *mut ibv_pd,
	pub ind_tbl_handle: c_int,
	pub ind_tbl_num: c_int,
	pub comp_mask: u32,
}

impl Default for ibv_exp_rwq_ind_table
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_rwq_ind_table
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_rwq_ind_table {{ context: {:?}, pd: {:?} }}", self.context, self.pd)
	}
}
