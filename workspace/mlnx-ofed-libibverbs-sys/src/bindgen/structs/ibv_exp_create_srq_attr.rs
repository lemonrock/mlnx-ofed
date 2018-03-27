// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_create_srq_attr
{
	pub base: ibv_srq_init_attr,
	pub srq_type: ibv_exp_srq_type,
	pub pd: *mut ibv_pd,
	pub comp_mask: u32,
	pub cq: *mut ibv_cq,
	pub xrcd: *mut ibv_xrcd,
	pub tm_cap: ibv_exp_tm_cap,
	pub dc_offload_params: *mut ibv_exp_srq_dc_offload_params,
	pub mp_wr: ibv_exp_mp_wr,
}

impl Default for ibv_exp_create_srq_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_create_srq_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_create_srq_attr {{ base: {:?}, srq_type: {:?}, pd: {:?}, cq: {:?}, xrcd: {:?}, tm_cap: {:?}, dc_offload_params: {:?}, mp_wr: {:?} }}", self.base, self.srq_type, self.pd, self.cq, self.xrcd, self.tm_cap, self.dc_offload_params, self.mp_wr)
	}
}
