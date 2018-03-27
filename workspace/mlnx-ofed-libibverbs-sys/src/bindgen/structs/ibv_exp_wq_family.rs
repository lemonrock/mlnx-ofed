// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ibv_exp_wq_family
{
	pub recv_sg_list: Option<unsafe extern "C" fn(wq: *mut ibv_exp_wq, sg_list: *mut ibv_sge, num_sg: u32) -> c_int>,
	pub recv_burst: Option<unsafe extern "C" fn(wq: *mut ibv_exp_wq, msg_list: *mut ibv_sge, num: u32) -> c_int>,
}

impl Default for ibv_exp_wq_family
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
