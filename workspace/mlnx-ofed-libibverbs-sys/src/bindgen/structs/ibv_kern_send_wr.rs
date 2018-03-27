// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_kern_send_wr
{
	pub wr_id: __u64,
	pub num_sge: __u32,
	pub opcode: __u32,
	pub send_flags: __u32,
	pub imm_data: __u32,
	pub wr: ibv_kern_send_wr__bindgen_ty_1,
	pub qp_type: ibv_kern_send_wr__bindgen_ty_2,
}

impl Default for ibv_kern_send_wr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_kern_send_wr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_kern_send_wr {{ wr: {:?}, qp_type: {:?} }}", self.wr, self.qp_type)
	}
}
