// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_srq_legacy
{
	pub context: *mut ibv_context,
	pub srq_context: *mut c_void,
	pub pd: *mut ibv_pd,
	pub handle: u32,
	pub events_completed: u32,
	pub xrc_srq_num_bin_compat: u32,
	pub xrc_domain_bin_compat: *mut ibv_xrc_domain,
	pub xrc_cq_bin_compat: *mut ibv_cq,
	pub mutex: pthread_mutex_t,
	pub cond: pthread_cond_t,
	pub ibv_srq: *mut c_void,
	pub xrc_srq_num: u32,
	pub xrc_domain: *mut ibv_xrc_domain,
	pub xrc_cq: *mut ibv_cq,
}

impl Default for ibv_srq_legacy
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_srq_legacy
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_srq_legacy {{ context: {:?}, srq_context: {:?}, pd: {:?}, xrc_domain_bin_compat: {:?}, xrc_cq_bin_compat: {:?}, ibv_srq: {:?}, xrc_domain: {:?}, xrc_cq: {:?} }}", self.context, self.srq_context, self.pd, self.xrc_domain_bin_compat, self.xrc_cq_bin_compat, self.ibv_srq, self.xrc_domain, self.xrc_cq)
	}
}
