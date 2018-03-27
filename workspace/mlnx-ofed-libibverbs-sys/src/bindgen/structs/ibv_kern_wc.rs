// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_kern_wc
{
	pub wr_id: __u64,
	pub status: __u32,
	pub opcode: __u32,
	pub vendor_err: __u32,
	pub byte_len: __u32,
	pub imm_data: __u32,
	pub qp_num: __u32,
	pub src_qp: __u32,
	pub wc_flags: __u32,
	pub pkey_index: __u16,
	pub slid: __u16,
	pub sl: __u8,
	pub dlid_path_bits: __u8,
	pub port_num: __u8,
	pub reserved: __u8,
}

impl Default for ibv_kern_wc
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_kern_wc
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_kern_wc {{  }}")
	}
}
