// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_open_xrcd
{
	pub command: __u32,
	pub in_words: __u16,
	pub out_words: __u16,
	pub response: __u64,
	pub fd: __u32,
	pub oflags: __u32,
	pub driver_data: __IncompleteArrayField<__u64>,
}

impl Default for ibv_open_xrcd
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_open_xrcd
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_open_xrcd {{ driver_data: {:?} }}", self.driver_data)
	}
}
