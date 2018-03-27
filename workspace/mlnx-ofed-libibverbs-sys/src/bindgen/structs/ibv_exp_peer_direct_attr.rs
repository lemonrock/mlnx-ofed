// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_peer_direct_attr
{
	pub peer_id: u64,
	pub buf_alloc: Option<unsafe extern "C" fn(attr: *mut ibv_exp_peer_buf_alloc_attr) -> *mut ibv_exp_peer_buf>,
	pub buf_release: Option<unsafe extern "C" fn(pb: *mut ibv_exp_peer_buf) -> c_int>,
	pub register_va: Option<unsafe extern "C" fn(start: *mut c_void, length: usize, peer_id: u64, pb: *mut ibv_exp_peer_buf) -> u64>,
	pub unregister_va: Option<unsafe extern "C" fn(target_id: u64, peer_id: u64) -> c_int>,
	pub caps: u64,
	pub peer_dma_op_map_len: usize,
	pub comp_mask: u32,
	pub version: u32,
}

impl Default for ibv_exp_peer_direct_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_peer_direct_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_peer_direct_attr {{ buf_alloc: {:?}, buf_release: {:?}, register_va: {:?}, unregister_va: {:?} }}", self.buf_alloc, self.buf_release, self.register_va, self.unregister_va)
	}
}
