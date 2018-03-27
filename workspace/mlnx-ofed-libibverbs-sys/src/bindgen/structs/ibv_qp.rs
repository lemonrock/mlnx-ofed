// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_qp
{
	pub context: *mut ibv_context,
	pub qp_context: *mut c_void,
	pub pd: *mut ibv_pd,
	pub send_cq: *mut ibv_cq,
	pub recv_cq: *mut ibv_cq,
	pub srq: *mut ibv_srq,
	pub handle: u32,
	pub qp_num: u32,
	pub state: ibv_qp_state,
	pub qp_type: ibv_qp_type,
	pub mutex: pthread_mutex_t,
	pub cond: pthread_cond_t,
	pub events_completed: u32,
}

impl Default for ibv_qp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_qp
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_qp {{ context: {:?}, qp_context: {:?}, pd: {:?}, send_cq: {:?}, recv_cq: {:?}, srq: {:?}, state: {:?}, qp_type: {:?} }}", self.context, self.qp_context, self.pd, self.send_cq, self.recv_cq, self.srq, self.state, self.qp_type)
	}
}
