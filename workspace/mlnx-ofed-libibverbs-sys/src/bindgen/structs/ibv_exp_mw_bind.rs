// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_mw_bind
{
	pub qp: *mut ibv_qp,
	pub mw: *mut ibv_mw,
	pub wr_id: u64,
	pub exp_send_flags: u64,
	pub bind_info: ibv_exp_mw_bind_info,
	pub comp_mask: u32,
}

impl Default for ibv_exp_mw_bind
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_mw_bind
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_mw_bind {{ qp: {:?}, mw: {:?}, bind_info: {:?} }}", self.qp, self.mw, self.bind_info)
	}
}
