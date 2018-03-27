// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_ops_wr
{
	pub wr_id: u64,
	pub next: *mut ibv_exp_ops_wr,
	pub opcode: ibv_exp_ops_wr_opcode,
	pub flags: c_int,
	pub tm: ibv_exp_ops_wr__bindgen_ty_1,
}

impl Default for ibv_exp_ops_wr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_ops_wr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_ops_wr {{ next: {:?}, opcode: {:?}, tm: {:?} }}", self.next, self.opcode, self.tm)
	}
}
