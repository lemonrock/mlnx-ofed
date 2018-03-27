// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_query_intf_params
{
	pub flags: u32,
	pub intf_scope: ibv_exp_intf_scope,
	pub vendor_guid: u64,
	pub intf: u32,
	pub intf_version: u32,
	pub obj: *mut c_void,
	pub family_params: *mut c_void,
	pub family_flags: u32,
	pub comp_mask: u32,
}

impl Default for ibv_exp_query_intf_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_query_intf_params
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_query_intf_params {{ intf_scope: {:?}, obj: {:?}, family_params: {:?} }}", self.intf_scope, self.obj, self.family_params)
	}
}
