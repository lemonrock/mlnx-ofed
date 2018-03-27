// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ibv_exp_cq_family_v1
{
	pub poll_cnt: Option<unsafe extern "C" fn(cq: *mut ibv_cq, max: u32) -> i32>,
	pub poll_length: Option<unsafe extern "C" fn(cq: *mut ibv_cq, buf: *mut c_void, inl: *mut u32) -> i32>,
	pub poll_length_flags: Option<unsafe extern "C" fn(cq: *mut ibv_cq, buf: *mut c_void, inl: *mut u32, flags: *mut u32) -> i32>,
	pub poll_length_flags_mp_rq: Option<unsafe extern "C" fn(cq: *mut ibv_cq, offset: *mut u32, flags: *mut u32) -> i32>,
	pub poll_length_flags_cvlan: Option<unsafe extern "C" fn(cq: *mut ibv_cq, buf: *mut c_void, inl: *mut u32, flags: *mut u32, vlan_tci: *mut u16) -> i32>,
	pub poll_length_flags_mp_rq_cvlan: Option<unsafe extern "C" fn(cq: *mut ibv_cq, offset: *mut u32, flags: *mut u32, vlan_tci: *mut u16) -> i32>,
	pub poll_length_flags_ts: Option<unsafe extern "C" fn(cq: *mut ibv_cq, buf: *mut c_void, inl: *mut u32, flags: *mut u32, timestamp: *mut u64) -> i32>,
	pub poll_length_flags_mp_rq_ts: Option<unsafe extern "C" fn(cq: *mut ibv_cq, offset: *mut u32, flags: *mut u32, timestamp: *mut u64) -> i32>,
	pub poll_length_flags_cvlan_ts: Option<unsafe extern "C" fn(cq: *mut ibv_cq, buf: *mut c_void, inl: *mut u32, flags: *mut u32, vlan_tci: *mut u16, timestamp: *mut u64) -> i32>,
	pub poll_length_flags_mp_rq_cvlan_ts: Option<unsafe extern "C" fn(cq: *mut ibv_cq, offset: *mut u32, flags: *mut u32, vlan_tci: *mut u16, timestamp: *mut u64) -> i32>,
}

impl Default for ibv_exp_cq_family_v1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
