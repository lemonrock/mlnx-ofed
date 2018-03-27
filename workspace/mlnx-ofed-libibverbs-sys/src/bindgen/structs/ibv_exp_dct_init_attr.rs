// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_dct_init_attr
{
	pub pd: *mut ibv_pd,
	pub cq: *mut ibv_cq,
	pub srq: *mut ibv_srq,
	pub dc_key: u64,
	pub port: u8,
	pub access_flags: u32,
	pub min_rnr_timer: u8,
	pub tclass: u8,
	pub flow_label: u32,
	pub mtu: ibv_mtu,
	pub pkey_index: u8,
	pub gid_index: u8,
	pub hop_limit: u8,
	pub inline_size: u32,
	pub create_flags: u32,
	pub comp_mask: u32,
}

impl Default for ibv_exp_dct_init_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_dct_init_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_dct_init_attr {{ pd: {:?}, cq: {:?}, srq: {:?}, mtu: {:?} }}", self.pd, self.cq, self.srq, self.mtu)
	}
}
