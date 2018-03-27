// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_send_wr__bindgen_ty_8__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1
{
	pub mem_repeat_block_list: *mut ibv_exp_mem_repeat_block,
	pub repeat_count: *mut usize,
	pub stride_dim: u32,
}

impl Default for ibv_exp_send_wr__bindgen_ty_8__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_send_wr__bindgen_ty_8__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_send_wr__bindgen_ty_8__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {{ mem_repeat_block_list: {:?}, repeat_count: {:?} }}", self.mem_repeat_block_list, self.repeat_count)
	}
}
