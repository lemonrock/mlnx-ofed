// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct rdma_addr
{
	pub __bindgen_anon_1: rdma_addr__bindgen_ty_1,
	pub __bindgen_anon_2: rdma_addr__bindgen_ty_2,
	pub addr: rdma_addr__bindgen_ty_3,
}

impl Default for rdma_addr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rdma_addr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rdma_addr {{ __bindgen_anon_1: {:?}, __bindgen_anon_2: {:?}, addr: {:?} }}", self.__bindgen_anon_1, self.__bindgen_anon_2, self.addr)
	}
}
