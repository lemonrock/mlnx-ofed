// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_send_wr
{
	pub wr_id: u64,
	pub next: *mut ibv_exp_send_wr,
	pub sg_list: *mut ibv_sge,
	pub num_sge: c_int,
	pub exp_opcode: ibv_exp_wr_opcode,
	pub reserved: c_int,
	pub ex: ibv_exp_send_wr__bindgen_ty_1,
	pub wr: ibv_exp_send_wr__bindgen_ty_2,
	pub __bindgen_anon_1: ibv_exp_send_wr__bindgen_ty_3,
	pub task: ibv_exp_send_wr__bindgen_ty_4,
	pub op: ibv_exp_send_wr__bindgen_ty_5,
	pub dc: ibv_exp_send_wr__bindgen_ty_6,
	pub __bindgen_anon_2: ibv_exp_send_wr__bindgen_ty_7,
	pub exp_send_flags: u64,
	pub comp_mask: u32,
	pub ext_op: ibv_exp_send_wr__bindgen_ty_8,
}

impl Default for ibv_exp_send_wr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_send_wr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_send_wr {{ next: {:?}, sg_list: {:?}, exp_opcode: {:?}, ex: {:?}, wr: {:?}, __bindgen_anon_1: {:?}, task: {:?}, op: {:?}, dc: {:?}, __bindgen_anon_2: {:?}, ext_op: {:?} }}", self.next, self.sg_list, self.exp_opcode, self.ex, self.wr, self.__bindgen_anon_1, self.task, self.op, self.dc, self.__bindgen_anon_2, self.ext_op)
	}
}
