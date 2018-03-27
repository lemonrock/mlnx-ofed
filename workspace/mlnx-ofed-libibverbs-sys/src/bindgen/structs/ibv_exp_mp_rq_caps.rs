// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_mp_rq_caps
{
	pub supported_qps: u32,
	pub allowed_shifts: u32,
	pub min_single_wqe_log_num_of_strides: u8,
	pub max_single_wqe_log_num_of_strides: u8,
	pub min_single_stride_log_num_of_bytes: u8,
	pub max_single_stride_log_num_of_bytes: u8,
}

impl Default for ibv_exp_mp_rq_caps
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_mp_rq_caps
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_mp_rq_caps {{  }}")
	}
}
