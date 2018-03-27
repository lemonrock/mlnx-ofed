// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_query_device_resp
{
	pub fw_ver: __u64,
	pub node_guid: __u64,
	pub sys_image_guid: __u64,
	pub max_mr_size: __u64,
	pub page_size_cap: __u64,
	pub vendor_id: __u32,
	pub vendor_part_id: __u32,
	pub hw_ver: __u32,
	pub max_qp: __u32,
	pub max_qp_wr: __u32,
	pub device_cap_flags: __u32,
	pub max_sge: __u32,
	pub max_sge_rd: __u32,
	pub max_cq: __u32,
	pub max_cqe: __u32,
	pub max_mr: __u32,
	pub max_pd: __u32,
	pub max_qp_rd_atom: __u32,
	pub max_ee_rd_atom: __u32,
	pub max_res_rd_atom: __u32,
	pub max_qp_init_rd_atom: __u32,
	pub max_ee_init_rd_atom: __u32,
	pub atomic_cap: __u32,
	pub max_ee: __u32,
	pub max_rdd: __u32,
	pub max_mw: __u32,
	pub max_raw_ipv6_qp: __u32,
	pub max_raw_ethy_qp: __u32,
	pub max_mcast_grp: __u32,
	pub max_mcast_qp_attach: __u32,
	pub max_total_mcast_qp_attach: __u32,
	pub max_ah: __u32,
	pub max_fmr: __u32,
	pub max_map_per_fmr: __u32,
	pub max_srq: __u32,
	pub max_srq_wr: __u32,
	pub max_srq_sge: __u32,
	pub max_pkeys: __u16,
	pub local_ca_ack_delay: __u8,
	pub phys_port_cnt: __u8,
	pub reserved: [__u8; 4usize],
}

impl Default for ibv_query_device_resp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_query_device_resp
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_query_device_resp {{ reserved: {:?} }}", self.reserved)
	}
}
