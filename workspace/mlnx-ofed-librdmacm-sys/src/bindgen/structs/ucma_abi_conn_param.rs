// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ucma_abi_conn_param
{
	pub qp_num: __u32,
	pub reserved: __u32,
	pub private_data: [__u8; 256usize],
	pub private_data_len: __u8,
	pub srq: __u8,
	pub responder_resources: __u8,
	pub initiator_depth: __u8,
	pub flow_control: __u8,
	pub retry_count: __u8,
	pub rnr_retry_count: __u8,
	pub valid: __u8,
}

impl Default for ucma_abi_conn_param
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ucma_abi_conn_param
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"ucma_abi_conn_param {{ private_data: [{}] }}",
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
				.collect::<String>()
		)
	}
}
