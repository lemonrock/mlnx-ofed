// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_device_attr
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
	pub reserved: c_int,
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
	pub exp_atomic_cap: ibv_exp_atomic_cap,
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
	pub comp_mask: u32,
	pub calc_cap: ibv_exp_device_calc_cap,
	pub timestamp_mask: u64,
	pub hca_core_clock: u64,
	pub exp_device_cap_flags: u64,
	pub max_dc_req_rd_atom: c_int,
	pub max_dc_res_rd_atom: c_int,
	pub inline_recv_sz: c_int,
	pub max_rss_tbl_sz: u32,
	pub ext_atom: ibv_exp_ext_atomics_params,
	pub umr_caps: ibv_exp_umr_caps,
	pub odp_caps: ibv_exp_odp_caps,
	pub max_dct: c_int,
	pub max_ctx_res_domain: c_int,
	pub rx_hash_caps: ibv_exp_rx_hash_caps,
	pub max_wq_type_rq: u32,
	pub max_device_ctx: c_int,
	pub mp_rq_caps: ibv_exp_mp_rq_caps,
	pub wq_vlan_offloads_cap: u16,
	pub ec_caps: ibv_exp_ec_caps,
	pub masked_atomic: ibv_exp_masked_atomic_params,
	pub rx_pad_end_addr_align: c_int,
	pub tso_caps: ibv_exp_tso_caps,
	pub packet_pacing_caps: ibv_exp_packet_pacing_caps,
	pub ec_w_mask: u32,
	pub ooo_caps: ibv_exp_ooo_caps,
	pub sw_parsing_caps: ibv_exp_sw_parsing_caps,
	pub odp_mr_max_size: u64,
	pub tm_caps: ibv_exp_tm_caps,
	pub tunnel_offloads_caps: u32,
	pub max_dm_size: u64,
	pub tunneled_atomic_caps: u32,
}

impl Default for ibv_exp_device_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_device_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"ibv_exp_device_attr {{ fw_ver: [{}], exp_atomic_cap: {:?}, calc_cap: {:?}, ext_atom: {:?}, umr_caps: {:?}, odp_caps: {:?}, rx_hash_caps: {:?}, mp_rq_caps: {:?}, ec_caps: {:?}, masked_atomic: {:?}, tso_caps: {:?}, packet_pacing_caps: {:?}, ooo_caps: {:?}, sw_parsing_caps: {:?}, tm_caps: {:?} }}",
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
			self.exp_atomic_cap,
			self.calc_cap,
			self.ext_atom,
			self.umr_caps,
			self.odp_caps,
			self.rx_hash_caps,
			self.mp_rq_caps,
			self.ec_caps,
			self.masked_atomic,
			self.tso_caps,
			self.packet_pacing_caps,
			self.ooo_caps,
			self.sw_parsing_caps,
			self.tm_caps
		)
	}
}
