// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_wq
{
	pub context: *mut ibv_context,
	pub wq_context: *mut c_void,
	pub handle: u32,
	pub pd: *mut ibv_pd,
	pub cq: *mut ibv_cq,
	pub srq: *mut ibv_srq,
	pub wq_num: u32,
	pub state: ibv_exp_wq_state,
	pub wq_type: ibv_exp_wq_type,
	pub comp_mask: u32,
}

impl Default for ibv_exp_wq
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_wq
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_wq {{ context: {:?}, wq_context: {:?}, pd: {:?}, cq: {:?}, srq: {:?}, state: {:?}, wq_type: {:?} }}", self.context, self.wq_context, self.pd, self.cq, self.srq, self.state, self.wq_type)
	}
}
