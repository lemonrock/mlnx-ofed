// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct verbs_srq
{
	pub srq: ibv_srq,
	pub comp_mask: u32,
	pub srq_type: ibv_srq_type,
	pub xrcd: *mut verbs_xrcd,
	pub cq: *mut ibv_cq,
	pub srq_num: u32,
}

impl Default for verbs_srq
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for verbs_srq
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "verbs_srq {{ srq: {:?}, srq_type: {:?}, xrcd: {:?}, cq: {:?} }}", self.srq, self.srq_type, self.xrcd, self.cq)
	}
}
