// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_wc
{
	pub wr_id: u64,
	pub status: ibv_wc_status,
	pub exp_opcode: ibv_exp_wc_opcode,
	pub vendor_err: u32,
	pub byte_len: u32,
	pub imm_data: u32,
	pub qp_num: u32,
	pub src_qp: u32,
	pub reserved: c_int,
	pub pkey_index: u16,
	pub slid: u16,
	pub sl: u8,
	pub dlid_path_bits: u8,
	pub timestamp: u64,
	pub qp: *mut ibv_qp,
	pub srq: *mut ibv_srq,
	pub dct: *mut ibv_exp_dct,
	pub exp_wc_flags: u64,
	pub tm_info: ibv_exp_wc__bindgen_ty_1,
}

impl Default for ibv_exp_wc
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_wc
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_wc {{ status: {:?}, exp_opcode: {:?}, qp: {:?}, srq: {:?}, dct: {:?}, tm_info: {:?} }}", self.status, self.exp_opcode, self.qp, self.srq, self.dct, self.tm_info)
	}
}
