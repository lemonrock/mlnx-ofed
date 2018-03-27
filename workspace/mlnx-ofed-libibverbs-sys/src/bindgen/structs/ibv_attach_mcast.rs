// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_attach_mcast
{
	pub command: __u32,
	pub in_words: __u16,
	pub out_words: __u16,
	pub gid: [__u8; 16usize],
	pub qp_handle: __u32,
	pub mlid: __u16,
	pub reserved: __u16,
	pub driver_data: __IncompleteArrayField<__u64>,
}

impl Default for ibv_attach_mcast
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_attach_mcast
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_attach_mcast {{ gid: {:?}, driver_data: {:?} }}", self.gid, self.driver_data)
	}
}
