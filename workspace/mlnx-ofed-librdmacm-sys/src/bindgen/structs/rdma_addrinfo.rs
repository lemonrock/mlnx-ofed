// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct rdma_addrinfo
{
	pub ai_flags: c_int,
	pub ai_family: c_int,
	pub ai_qp_type: c_int,
	pub ai_port_space: c_int,
	pub ai_src_len: socklen_t,
	pub ai_dst_len: socklen_t,
	pub ai_src_addr: *mut sockaddr,
	pub ai_dst_addr: *mut sockaddr,
	pub ai_src_canonname: *mut c_char,
	pub ai_dst_canonname: *mut c_char,
	pub ai_route_len: usize,
	pub ai_route: *mut c_void,
	pub ai_connect_len: usize,
	pub ai_connect: *mut c_void,
	pub ai_next: *mut rdma_addrinfo,
}

impl Default for rdma_addrinfo
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rdma_addrinfo
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rdma_addrinfo {{ ai_src_addr: {:?}, ai_dst_addr: {:?}, ai_src_canonname: {:?}, ai_dst_canonname: {:?}, ai_route: {:?}, ai_connect: {:?}, ai_next: {:?} }}", self.ai_src_addr, self.ai_dst_addr, self.ai_src_canonname, self.ai_dst_canonname, self.ai_route, self.ai_connect, self.ai_next)
	}
}
