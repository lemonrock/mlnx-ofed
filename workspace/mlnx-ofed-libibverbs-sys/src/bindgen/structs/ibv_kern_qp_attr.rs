// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_kern_qp_attr
{
	pub qp_attr_mask: __u32,
	pub qp_state: __u32,
	pub cur_qp_state: __u32,
	pub path_mtu: __u32,
	pub path_mig_state: __u32,
	pub qkey: __u32,
	pub rq_psn: __u32,
	pub sq_psn: __u32,
	pub dest_qp_num: __u32,
	pub qp_access_flags: __u32,
	pub ah_attr: ibv_kern_ah_attr,
	pub alt_ah_attr: ibv_kern_ah_attr,
	pub max_send_wr: __u32,
	pub max_recv_wr: __u32,
	pub max_send_sge: __u32,
	pub max_recv_sge: __u32,
	pub max_inline_data: __u32,
	pub pkey_index: __u16,
	pub alt_pkey_index: __u16,
	pub en_sqd_async_notify: __u8,
	pub sq_draining: __u8,
	pub max_rd_atomic: __u8,
	pub max_dest_rd_atomic: __u8,
	pub min_rnr_timer: __u8,
	pub port_num: __u8,
	pub timeout: __u8,
	pub retry_cnt: __u8,
	pub rnr_retry: __u8,
	pub alt_port_num: __u8,
	pub alt_timeout: __u8,
	pub reserved: [__u8; 5usize],
}

impl Default for ibv_kern_qp_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_kern_qp_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_kern_qp_attr {{ ah_attr: {:?}, alt_ah_attr: {:?}, reserved: {:?} }}", self.ah_attr, self.alt_ah_attr, self.reserved)
	}
}
