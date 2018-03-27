// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_kern_ah_attr
{
	pub grh: ibv_kern_global_route,
	pub dlid: __u16,
	pub sl: __u8,
	pub src_path_bits: __u8,
	pub static_rate: __u8,
	pub is_global: __u8,
	pub port_num: __u8,
	pub reserved: __u8,
}

impl Default for ibv_kern_ah_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_kern_ah_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_kern_ah_attr {{ grh: {:?} }}", self.grh)
	}
}
