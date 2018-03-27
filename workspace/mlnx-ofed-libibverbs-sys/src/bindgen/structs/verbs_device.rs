// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct verbs_device
{
	pub device: ibv_device,
	pub sz: usize,
	pub size_of_context: usize,
	pub init_context: Option<unsafe extern "C" fn(device: *mut verbs_device, ctx: *mut ibv_context, cmd_fd: c_int) -> c_int>,
	pub uninit_context: Option<unsafe extern "C" fn(device: *mut verbs_device, ctx: *mut ibv_context)>,
	pub refcount: c_int,
	pub reflock: pthread_mutex_t,
	pub verbs_uninit_func: Option<unsafe extern "C" fn(device: *mut verbs_device)>,
}

impl Default for verbs_device
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for verbs_device
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "verbs_device {{ device: {:?}, init_context: {:?}, uninit_context: {:?}, verbs_uninit_func: {:?} }}", self.device, self.init_context, self.uninit_context, self.verbs_uninit_func)
	}
}
