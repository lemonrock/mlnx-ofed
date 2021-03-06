// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibv_exp_task__bindgen_ty_1
{
	pub qp: *mut ibv_qp,
	pub __bindgen_anon_1: ibv_exp_task__bindgen_ty_1__bindgen_ty_1,
}

impl Default for ibv_exp_task__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_task__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_task__bindgen_ty_1 {{ qp: {:?}, __bindgen_anon_1: {:?} }}", self.qp, self.__bindgen_anon_1)
	}
}
