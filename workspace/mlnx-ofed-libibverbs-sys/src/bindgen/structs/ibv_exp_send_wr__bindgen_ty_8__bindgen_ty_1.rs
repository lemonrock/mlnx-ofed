// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_send_wr__bindgen_ty_8__bindgen_ty_1
{
	pub umr_type: u32,
	pub memory_objects: *mut ibv_exp_mkey_list_container,
	pub exp_access: u64,
	pub modified_mr: *mut ibv_mr,
	pub base_addr: u64,
	pub num_mrs: u32,
	pub mem_list: ibv_exp_send_wr__bindgen_ty_8__bindgen_ty_1__bindgen_ty_1,
}

impl Default for ibv_exp_send_wr__bindgen_ty_8__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_send_wr__bindgen_ty_8__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_send_wr__bindgen_ty_8__bindgen_ty_1 {{ memory_objects: {:?}, modified_mr: {:?}, mem_list: {:?} }}", self.memory_objects, self.modified_mr, self.mem_list)
	}
}
