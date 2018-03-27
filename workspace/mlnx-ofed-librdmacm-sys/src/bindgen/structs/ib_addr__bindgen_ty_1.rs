// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ib_addr__bindgen_ty_1
{
	pub uib_addr8: __BindgenUnionField<[__u8; 16usize]>,
	pub uib_addr16: __BindgenUnionField<[__be16; 8usize]>,
	pub uib_addr32: __BindgenUnionField<[__be32; 4usize]>,
	pub uib_addr64: __BindgenUnionField<[__be64; 2usize]>,
	pub bindgen_union_field: [u64; 2usize],
}

impl Default for ib_addr__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ib_addr__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ib_addr__bindgen_ty_1 {{ union }}")
	}
}
