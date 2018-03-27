// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_send_wr__bindgen_ty_4
{
	pub rdma: __BindgenUnionField<ibv_exp_send_wr__bindgen_ty_4__bindgen_ty_1>,
	pub atomic: __BindgenUnionField<ibv_exp_send_wr__bindgen_ty_4__bindgen_ty_2>,
	pub cqe_wait: __BindgenUnionField<ibv_exp_send_wr__bindgen_ty_4__bindgen_ty_3>,
	pub wqe_enable: __BindgenUnionField<ibv_exp_send_wr__bindgen_ty_4__bindgen_ty_4>,
	pub bindgen_union_field: [u64; 4usize],
}

impl Default for ibv_exp_send_wr__bindgen_ty_4
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_send_wr__bindgen_ty_4
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_send_wr__bindgen_ty_4 {{ union }}")
	}
}
