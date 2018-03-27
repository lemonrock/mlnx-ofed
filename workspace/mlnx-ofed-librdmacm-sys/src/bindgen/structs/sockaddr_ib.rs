// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct sockaddr_ib
{
	pub sib_family: c_ushort,
	pub sib_pkey: __be16,
	pub sib_flowinfo: __be32,
	pub sib_addr: ib_addr,
	pub sib_sid: __be64,
	pub sib_sid_mask: __be64,
	pub sib_scope_id: __u64,
}

impl Default for sockaddr_ib
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for sockaddr_ib
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "sockaddr_ib {{ sib_addr: {:?} }}", self.sib_addr)
	}
}
