// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_mini_cqe8__bindgen_ty_1
{
	pub rx_hash_result: __BindgenUnionField<u32>,
	pub checksum: __BindgenUnionField<u32>,
	pub s_wqe_info: __BindgenUnionField<mlx5_mini_cqe8__bindgen_ty_1__bindgen_ty_1>,
	pub bindgen_union_field: u32,
}

impl Default for mlx5_mini_cqe8__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_mini_cqe8__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_mini_cqe8__bindgen_ty_1 {{ union }}")
	}
}
