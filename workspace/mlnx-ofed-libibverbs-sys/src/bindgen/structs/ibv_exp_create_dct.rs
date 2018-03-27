// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_create_dct
{
	pub hdr: ex_hdr,
	pub comp_mask: __u64,
	pub user_handle: __u64,
	pub pd_handle: __u32,
	pub cq_handle: __u32,
	pub srq_handle: __u32,
	pub access_flags: __u32,
	pub dc_key: __u64,
	pub flow_label: __u32,
	pub min_rnr_timer: __u8,
	pub tclass: __u8,
	pub port: __u8,
	pub pkey_index: __u8,
	pub gid_index: __u8,
	pub hop_limit: __u8,
	pub mtu: __u8,
	pub rsvd0: __u8,
	pub create_flags: __u32,
	pub inline_size: __u32,
	pub rsvd1: __u32,
	pub driver_data: __IncompleteArrayField<__u64>,
}

impl Default for ibv_exp_create_dct
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_create_dct
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_create_dct {{ hdr: {:?}, driver_data: {:?} }}", self.hdr, self.driver_data)
	}
}
