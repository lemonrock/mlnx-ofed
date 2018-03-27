// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_create_qp_resp
{
	pub qp_handle: __u32,
	pub qpn: __u32,
	pub max_send_wr: __u32,
	pub max_recv_wr: __u32,
	pub max_send_sge: __u32,
	pub max_recv_sge: __u32,
	pub max_inline_data: __u32,
	pub reserved: __u32,
}

impl Default for ibv_create_qp_resp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_create_qp_resp
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_create_qp_resp {{  }}")
	}
}
