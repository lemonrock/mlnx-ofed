// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub union ibv_async_event__bindgen_ty_1
{
	pub cq: *mut ibv_cq,
	pub qp: *mut ibv_qp,
	pub srq: *mut ibv_srq,
	pub dct: *mut ibv_exp_dct,
	pub port_num: c_int,
	pub xrc_qp_num: u32,
	_bindgen_union_align: u64,
}

impl Default for ibv_async_event__bindgen_ty_1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_async_event__bindgen_ty_1
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_async_event__bindgen_ty_1 {{ union }}")
	}
}
