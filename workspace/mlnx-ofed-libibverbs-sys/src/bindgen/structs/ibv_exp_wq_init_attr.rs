// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_wq_init_attr
{
	pub wq_context: *mut c_void,
	pub wq_type: ibv_exp_wq_type,
	pub max_recv_wr: u32,
	pub max_recv_sge: u32,
	pub pd: *mut ibv_pd,
	pub cq: *mut ibv_cq,
	pub srq: *mut ibv_srq,
	pub comp_mask: u32,
	pub res_domain: *mut ibv_exp_res_domain,
	pub mp_rq: ibv_exp_wq_mp_rq,
	pub vlan_offloads: u16,
	pub flags: u64,
}

impl Default for ibv_exp_wq_init_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_wq_init_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_wq_init_attr {{ wq_context: {:?}, wq_type: {:?}, pd: {:?}, cq: {:?}, srq: {:?}, res_domain: {:?}, mp_rq: {:?} }}", self.wq_context, self.wq_type, self.pd, self.cq, self.srq, self.res_domain, self.mp_rq)
	}
}
