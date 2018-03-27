// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_create_qp
{
	pub command: __u32,
	pub in_words: __u16,
	pub out_words: __u16,
	pub response: __u64,
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
	pub driver_data: __IncompleteArrayField<__u64>,
}

impl Default for ibv_create_qp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_create_qp
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_create_qp {{ driver_data: {:?} }}", self.driver_data)
	}
}
