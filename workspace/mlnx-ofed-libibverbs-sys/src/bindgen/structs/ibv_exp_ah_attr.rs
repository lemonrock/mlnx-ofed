// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_ah_attr
{
	pub grh: ibv_global_route,
	pub dlid: u16,
	pub sl: u8,
	pub src_path_bits: u8,
	pub static_rate: u8,
	pub is_global: u8,
	pub port_num: u8,
	pub comp_mask: u32,
	pub ll_address: ibv_exp_ah_attr__bindgen_ty_1,
	pub vid: u16,
}

impl Default for ibv_exp_ah_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_ah_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_ah_attr {{ grh: {:?}, ll_address: {:?} }}", self.grh, self.ll_address)
	}
}
