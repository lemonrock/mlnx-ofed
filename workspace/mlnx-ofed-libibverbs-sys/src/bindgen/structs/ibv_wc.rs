// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_wc
{
	pub wr_id: u64,
	pub status: ibv_wc_status,
	pub opcode: ibv_wc_opcode,
	pub vendor_err: u32,
	pub byte_len: u32,
	pub imm_data: u32,
	pub qp_num: u32,
	pub src_qp: u32,
	pub wc_flags: c_int,
	pub pkey_index: u16,
	pub slid: u16,
	pub sl: u8,
	pub dlid_path_bits: u8,
}

impl Default for ibv_wc
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_wc
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_wc {{ status: {:?}, opcode: {:?} }}", self.status, self.opcode)
	}
}
