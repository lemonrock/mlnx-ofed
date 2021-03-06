// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_qp_attr
{
	pub qp_state: ibv_qp_state,
	pub cur_qp_state: ibv_qp_state,
	pub path_mtu: ibv_mtu,
	pub path_mig_state: ibv_mig_state,
	pub qkey: u32,
	pub rq_psn: u32,
	pub sq_psn: u32,
	pub dest_qp_num: u32,
	pub qp_access_flags: c_int,
	pub cap: ibv_qp_cap,
	pub ah_attr: ibv_ah_attr,
	pub alt_ah_attr: ibv_ah_attr,
	pub pkey_index: u16,
	pub alt_pkey_index: u16,
	pub en_sqd_async_notify: u8,
	pub sq_draining: u8,
	pub max_rd_atomic: u8,
	pub max_dest_rd_atomic: u8,
	pub min_rnr_timer: u8,
	pub port_num: u8,
	pub timeout: u8,
	pub retry_cnt: u8,
	pub rnr_retry: u8,
	pub alt_port_num: u8,
	pub alt_timeout: u8,
	pub dct_key: u64,
	pub comp_mask: u32,
	pub flow_entropy: u32,
	pub rate_limit: u32,
	pub burst_info: ibv_exp_burst_info,
}

impl Default for ibv_exp_qp_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_qp_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_qp_attr {{ qp_state: {:?}, cur_qp_state: {:?}, path_mtu: {:?}, path_mig_state: {:?}, cap: {:?}, ah_attr: {:?}, alt_ah_attr: {:?}, burst_info: {:?} }}", self.qp_state, self.cur_qp_state, self.path_mtu, self.path_mig_state, self.cap, self.ah_attr, self.alt_ah_attr, self.burst_info)
	}
}
