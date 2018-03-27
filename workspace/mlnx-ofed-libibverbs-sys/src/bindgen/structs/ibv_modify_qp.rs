// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_modify_qp
{
	pub command: __u32,
	pub in_words: __u16,
	pub out_words: __u16,
	pub dest: ibv_qp_dest,
	pub alt_dest: ibv_qp_dest,
	pub qp_handle: __u32,
	pub attr_mask: __u32,
	pub qkey: __u32,
	pub rq_psn: __u32,
	pub sq_psn: __u32,
	pub dest_qp_num: __u32,
	pub qp_access_flags: __u32,
	pub pkey_index: __u16,
	pub alt_pkey_index: __u16,
	pub qp_state: __u8,
	pub cur_qp_state: __u8,
	pub path_mtu: __u8,
	pub path_mig_state: __u8,
	pub en_sqd_async_notify: __u8,
	pub max_rd_atomic: __u8,
	pub max_dest_rd_atomic: __u8,
	pub min_rnr_timer: __u8,
	pub port_num: __u8,
	pub timeout: __u8,
	pub retry_cnt: __u8,
	pub rnr_retry: __u8,
	pub alt_port_num: __u8,
	pub alt_timeout: __u8,
	pub reserved: [__u8; 2usize],
	pub driver_data: __IncompleteArrayField<__u64>,
}

impl Default for ibv_modify_qp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_modify_qp
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_modify_qp {{ dest: {:?}, alt_dest: {:?}, reserved: {:?}, driver_data: {:?} }}", self.dest, self.alt_dest, self.reserved, self.driver_data)
	}
}
