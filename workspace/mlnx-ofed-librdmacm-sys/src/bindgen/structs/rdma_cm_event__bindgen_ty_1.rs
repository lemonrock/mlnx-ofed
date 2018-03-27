// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct rdma_cm_event__bindgen_ty_1
{
	pub conn: __BindgenUnionField<rdma_conn_param>,
	pub ud: __BindgenUnionField<rdma_ud_param>,
	pub bindgen_union_field: [u64; 7usize],
}

impl Default for rdma_cm_event__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rdma_cm_event__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rdma_cm_event__bindgen_ty_1 {{ union }}")
	}
}
