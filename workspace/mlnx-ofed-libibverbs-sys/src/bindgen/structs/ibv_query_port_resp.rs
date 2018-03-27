// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_query_port_resp
{
	pub port_cap_flags: __u32,
	pub max_msg_sz: __u32,
	pub bad_pkey_cntr: __u32,
	pub qkey_viol_cntr: __u32,
	pub gid_tbl_len: __u32,
	pub pkey_tbl_len: __u16,
	pub lid: __u16,
	pub sm_lid: __u16,
	pub state: __u8,
	pub max_mtu: __u8,
	pub active_mtu: __u8,
	pub lmc: __u8,
	pub max_vl_num: __u8,
	pub sm_sl: __u8,
	pub subnet_timeout: __u8,
	pub init_type_reply: __u8,
	pub active_width: __u8,
	pub active_speed: __u8,
	pub phys_state: __u8,
	pub link_layer: __u8,
	pub reserved: [__u8; 2usize],
}

impl Default for ibv_query_port_resp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_query_port_resp
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_query_port_resp {{ reserved: {:?} }}", self.reserved)
	}
}
