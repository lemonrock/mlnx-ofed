// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_dct
{
	pub context: *mut ibv_context,
	pub handle: u32,
	pub dct_num: u32,
	pub pd: *mut ibv_pd,
	pub srq: *mut ibv_srq,
	pub cq: *mut ibv_cq,
	pub mutex: pthread_mutex_t,
	pub cond: pthread_cond_t,
	pub events_completed: u32,
}

impl Default for ibv_exp_dct
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_dct
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_dct {{ context: {:?}, pd: {:?}, srq: {:?}, cq: {:?} }}", self.context, self.pd, self.srq, self.cq)
	}
}
