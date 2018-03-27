// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_qp_init_attr_ex
{
	pub qp_context: *mut c_void,
	pub send_cq: *mut ibv_cq,
	pub recv_cq: *mut ibv_cq,
	pub srq: *mut ibv_srq,
	pub cap: ibv_qp_cap,
	pub qp_type: ibv_qp_type,
	pub sq_sig_all: c_int,
	pub comp_mask: u32,
	pub pd: *mut ibv_pd,
	pub xrcd: *mut ibv_xrcd,
}

impl Default for ibv_qp_init_attr_ex
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_qp_init_attr_ex
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_qp_init_attr_ex {{ qp_context: {:?}, send_cq: {:?}, recv_cq: {:?}, srq: {:?}, cap: {:?}, qp_type: {:?}, pd: {:?}, xrcd: {:?} }}", self.qp_context, self.send_cq, self.recv_cq, self.srq, self.cap, self.qp_type, self.pd, self.xrcd)
	}
}
