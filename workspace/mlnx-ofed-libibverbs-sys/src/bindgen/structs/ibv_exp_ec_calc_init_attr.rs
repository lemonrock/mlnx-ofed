// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_ec_calc_init_attr
{
	pub comp_mask: u32,
	pub max_inflight_calcs: u32,
	pub k: c_int,
	pub m: c_int,
	pub w: c_int,
	pub max_data_sge: c_int,
	pub max_code_sge: c_int,
	pub encode_matrix: *mut u8,
	pub affinity_hint: c_int,
	pub polling: c_int,
}

impl Default for ibv_exp_ec_calc_init_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_ec_calc_init_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_ec_calc_init_attr {{ encode_matrix: {:?} }}", self.encode_matrix)
	}
}
