// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ucma_abi_query_addr_resp
{
	pub node_guid: __u64,
	pub port_num: __u8,
	pub reserved: __u8,
	pub pkey: __u16,
	pub src_size: __u16,
	pub dst_size: __u16,
	pub src_addr: sockaddr_storage,
	pub dst_addr: sockaddr_storage,
}

impl Default for ucma_abi_query_addr_resp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ucma_abi_query_addr_resp
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ucma_abi_query_addr_resp {{  }}")
	}
}
