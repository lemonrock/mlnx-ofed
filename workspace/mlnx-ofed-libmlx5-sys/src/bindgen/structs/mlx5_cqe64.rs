// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_cqe64
{
	pub __bindgen_anon_1: mlx5_cqe64__bindgen_ty_1,
	pub srqn_uidx: u32,
	pub imm_inval_pkey: u32,
	pub app: u8,
	pub app_op: u8,
	pub app_info: u16,
	pub byte_cnt: u32,
	pub timestamp: __be64,
	pub __bindgen_anon_2: mlx5_cqe64__bindgen_ty_2,
	pub wqe_counter: u16,
	pub signature: u8,
	pub op_own: u8,
}

impl Default for mlx5_cqe64
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_cqe64
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_cqe64 {{ __bindgen_anon_1: {:?}, __bindgen_anon_2: {:?} }}", self.__bindgen_anon_1, self.__bindgen_anon_2)
	}
}
