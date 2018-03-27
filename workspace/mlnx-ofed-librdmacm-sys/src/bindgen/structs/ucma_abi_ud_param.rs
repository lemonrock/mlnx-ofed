// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ucma_abi_ud_param
{
	pub qp_num: __u32,
	pub qkey: __u32,
	pub ah_attr: ibv_kern_ah_attr,
	pub private_data: [__u8; 256usize],
	pub private_data_len: __u8,
	pub reserved: [__u8; 7usize],
	pub reserved2: [__u8; 4usize],
}

impl Default for ucma_abi_ud_param
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ucma_abi_ud_param
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"ucma_abi_ud_param {{ private_data: [{}], reserved: {:?}, reserved2: {:?} }}",
			self.private_data
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>(),
			self.reserved,
			self.reserved2
		)
	}
}
