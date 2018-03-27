// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_device_attr
{
	pub fw_ver: [c_char; 64usize],
	pub node_guid: u64,
	pub sys_image_guid: u64,
	pub max_mr_size: u64,
	pub page_size_cap: u64,
	pub vendor_id: u32,
	pub vendor_part_id: u32,
	pub hw_ver: u32,
	pub max_qp: c_int,
	pub max_qp_wr: c_int,
	pub device_cap_flags: u32,
	pub max_sge: c_int,
	pub max_sge_rd: c_int,
	pub max_cq: c_int,
	pub max_cqe: c_int,
	pub max_mr: c_int,
	pub max_pd: c_int,
	pub max_qp_rd_atom: c_int,
	pub max_ee_rd_atom: c_int,
	pub max_res_rd_atom: c_int,
	pub max_qp_init_rd_atom: c_int,
	pub max_ee_init_rd_atom: c_int,
	pub atomic_cap: ibv_atomic_cap,
	pub max_ee: c_int,
	pub max_rdd: c_int,
	pub max_mw: c_int,
	pub max_raw_ipv6_qp: c_int,
	pub max_raw_ethy_qp: c_int,
	pub max_mcast_grp: c_int,
	pub max_mcast_qp_attach: c_int,
	pub max_total_mcast_qp_attach: c_int,
	pub max_ah: c_int,
	pub max_fmr: c_int,
	pub max_map_per_fmr: c_int,
	pub max_srq: c_int,
	pub max_srq_wr: c_int,
	pub max_srq_sge: c_int,
	pub max_pkeys: u16,
	pub local_ca_ack_delay: u8,
	pub phys_port_cnt: u8,
}

impl Default for ibv_device_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_device_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"ibv_device_attr {{ fw_ver: [{}], atomic_cap: {:?} }}",
			self.fw_ver
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>(),
			self.atomic_cap
		)
	}
}
