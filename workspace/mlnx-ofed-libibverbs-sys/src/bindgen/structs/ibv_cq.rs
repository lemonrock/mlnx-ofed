// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_cq
{
	pub context: *mut ibv_context,
	pub channel: *mut ibv_comp_channel,
	pub cq_context: *mut c_void,
	pub handle: u32,
	pub cqe: c_int,
	pub mutex: pthread_mutex_t,
	pub cond: pthread_cond_t,
	pub comp_events_completed: u32,
	pub async_events_completed: u32,
}

impl Default for ibv_cq
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_cq
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_cq {{ context: {:?}, channel: {:?}, cq_context: {:?} }}", self.context, self.channel, self.cq_context)
	}
}
