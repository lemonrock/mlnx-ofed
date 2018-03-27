// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_srq_dc_offload_params
{
	pub pkey_index: u16,
	pub path_mtu: ibv_mtu,
	pub sl: u8,
	pub max_rd_atomic: u8,
	pub min_rnr_timer: u8,
	pub timeout: u8,
	pub retry_cnt: u8,
	pub rnr_retry: u8,
	pub dct_key: u64,
	pub ooo_caps: u32,
	pub comp_mask: u32,
}

impl Default for ibv_exp_srq_dc_offload_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_srq_dc_offload_params
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_srq_dc_offload_params {{ path_mtu: {:?} }}", self.path_mtu)
	}
}
