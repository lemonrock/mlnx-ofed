// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ucma_abi_query_route_resp
{
	pub node_guid: __u64,
	pub ib_route: [ibv_kern_path_rec; 2usize],
	pub src_addr: sockaddr_in6,
	pub dst_addr: sockaddr_in6,
	pub num_paths: __u32,
	pub port_num: __u8,
	pub reserved: [__u8; 3usize],
}

impl Default for ucma_abi_query_route_resp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ucma_abi_query_route_resp
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ucma_abi_query_route_resp {{ ib_route: {:?}, reserved: {:?} }}", self.ib_route, self.reserved)
	}
}
