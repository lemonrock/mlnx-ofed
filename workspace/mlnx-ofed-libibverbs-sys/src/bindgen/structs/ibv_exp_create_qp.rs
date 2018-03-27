// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_create_qp
{
	pub hdr: ex_hdr,
	pub comp_mask: __u64,
	pub user_handle: __u64,
	pub pd_handle: __u32,
	pub send_cq_handle: __u32,
	pub recv_cq_handle: __u32,
	pub srq_handle: __u32,
	pub max_send_wr: __u32,
	pub max_recv_wr: __u32,
	pub max_send_sge: __u32,
	pub max_recv_sge: __u32,
	pub max_inline_data: __u32,
	pub sq_sig_all: __u8,
	pub qp_type: __u8,
	pub is_srq: __u8,
	pub reserved: __u8,
	pub qp_cap_flags: __u64,
	pub max_inl_recv: __u32,
	pub reserved1: __u32,
	pub qpg: ibv_create_qpg,
	pub max_inl_send_klms: __u64,
	pub rx_hash_info: ibv_exp_create_qp__bindgen_ty_1,
	pub port_num: __u8,
	pub reserved_2: [__u8; 7usize],
	pub driver_data: __IncompleteArrayField<__u64>,
}

impl Default for ibv_exp_create_qp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_create_qp
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_create_qp {{ hdr: {:?}, qpg: {:?}, rx_hash_info: {:?}, reserved_2: {:?}, driver_data: {:?} }}", self.hdr, self.qpg, self.rx_hash_info, self.reserved_2, self.driver_data)
	}
}
