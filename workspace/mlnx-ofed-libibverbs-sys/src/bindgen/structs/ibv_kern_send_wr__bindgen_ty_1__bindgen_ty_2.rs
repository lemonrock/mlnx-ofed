// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_kern_send_wr__bindgen_ty_1__bindgen_ty_2
{
	pub remote_addr: __u64,
	pub compare_add: __u64,
	pub swap: __u64,
	pub rkey: __u32,
	pub reserved: __u32,
}

impl Default for ibv_kern_send_wr__bindgen_ty_1__bindgen_ty_2
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_kern_send_wr__bindgen_ty_1__bindgen_ty_2
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_kern_send_wr__bindgen_ty_1__bindgen_ty_2 {{  }}")
	}
}
