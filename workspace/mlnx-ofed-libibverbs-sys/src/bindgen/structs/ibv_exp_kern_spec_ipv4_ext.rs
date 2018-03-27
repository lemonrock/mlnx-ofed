// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_kern_spec_ipv4_ext
{
	pub type_: __u32,
	pub size: __u16,
	pub reserved: __u16,
	pub val: ibv_exp_kern_ipv4_ext_filter,
	pub mask: ibv_exp_kern_ipv4_ext_filter,
}

impl Default for ibv_exp_kern_spec_ipv4_ext
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_kern_spec_ipv4_ext
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_kern_spec_ipv4_ext {{ val: {:?}, mask: {:?} }}", self.val, self.mask)
	}
}