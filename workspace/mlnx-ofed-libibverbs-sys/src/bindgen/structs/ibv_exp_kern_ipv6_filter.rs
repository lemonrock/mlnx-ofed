// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct ibv_exp_kern_ipv6_filter
{
	pub src_ip: [__u8; 16usize],
	pub dst_ip: [__u8; 16usize],
}

impl Default for ibv_exp_kern_ipv6_filter
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_kern_ipv6_filter
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_kern_ipv6_filter {{ src_ip: {:?}, dst_ip: {:?} }}", self.src_ip, self.dst_ip)
	}
}