// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_query_device_resp
{
	pub comp_mask: __u64,
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
	pub exp_atomic_cap: __u32,
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
	pub timestamp_mask: __u64,
	pub hca_core_clock: __u64,
	pub device_cap_flags2: __u64,
	pub dc_rd_req: __u32,
	pub dc_rd_res: __u32,
	pub inline_recv_sz: __u32,
	pub max_rss_tbl_sz: __u32,
	pub log_atomic_arg_sizes: __u64,
	pub max_fa_bit_boundary: __u32,
	pub log_max_atomic_inline: __u32,
	pub umr_caps: ibv_exp_umr_caps_resp,
	pub odp_caps: ibv_exp_odp_caps_resp,
	pub max_dct: __u32,
	pub max_ctx_res_domain: __u32,
	pub rx_hash: ibv_exp_rx_hash_caps_resp,
	pub max_wq_type_rq: __u32,
	pub max_device_ctx: __u32,
	pub mp_rq_caps: ibv_exp_mp_rq_caps_resp,
	pub wq_vlan_offloads_cap: __u16,
	pub reserved1: [__u8; 2usize],
	pub ec_w_mask: __u32,
	pub ec_caps: ibv_exp_ec_caps_resp,
	pub masked_atomic_caps: ibv_exp_masked_atomic_caps,
	pub rx_pad_end_addr_align: __u16,
	pub reserved2: [__u8; 6usize],
	pub tso_caps: ibv_exp_lso_caps_resp,
	pub packet_pacing_caps: ibv_exp_packet_pacing_caps_resp,
	pub ooo_caps: ibv_exp_ooo_caps_resp,
	pub sw_parsing_caps: ibv_exp_sw_parsing_caps_resp,
	pub odp_mr_max_size: __u64,
	pub tm_caps: ibv_exp_tm_caps_resp,
	pub tunnel_offloads_caps: __u32,
	pub tunneled_atomic_caps: __u32,
	pub max_dm_size: __u64,
}

impl Default for ibv_exp_query_device_resp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_query_device_resp
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_query_device_resp {{ reserved: {:?}, umr_caps: {:?}, odp_caps: {:?}, rx_hash: {:?}, mp_rq_caps: {:?}, reserved1: {:?}, ec_caps: {:?}, masked_atomic_caps: {:?}, reserved2: {:?}, tso_caps: {:?}, packet_pacing_caps: {:?}, ooo_caps: {:?}, sw_parsing_caps: {:?}, tm_caps: {:?} }}", self.reserved, self.umr_caps, self.odp_caps, self.rx_hash, self.mp_rq_caps, self.reserved1, self.ec_caps, self.masked_atomic_caps, self.reserved2, self.tso_caps, self.packet_pacing_caps, self.ooo_caps, self.sw_parsing_caps, self.tm_caps)
	}
}
