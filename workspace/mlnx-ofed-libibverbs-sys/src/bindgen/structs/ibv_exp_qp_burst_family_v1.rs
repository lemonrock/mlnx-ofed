// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ibv_exp_qp_burst_family_v1
{
	pub send_pending: Option<unsafe extern "C" fn(qp: *mut ibv_qp, addr: u64, length: u32, lkey: u32, flags: u32) -> c_int>,
	pub send_pending_inline: Option<unsafe extern "C" fn(qp: *mut ibv_qp, addr: *mut c_void, length: u32, flags: u32) -> c_int>,
	pub send_pending_sg_list: Option<unsafe extern "C" fn(qp: *mut ibv_qp, sg_list: *mut ibv_sge, num: u32, flags: u32) -> c_int>,
	pub send_flush: Option<unsafe extern "C" fn(qp: *mut ibv_qp) -> c_int>,
	pub send_burst: Option<unsafe extern "C" fn(qp: *mut ibv_qp, msg_list: *mut ibv_sge, num: u32, flags: u32) -> c_int>,
	pub recv_burst: Option<unsafe extern "C" fn(qp: *mut ibv_qp, msg_list: *mut ibv_sge, num: u32) -> c_int>,
	pub send_pending_vlan: Option<unsafe extern "C" fn(qp: *mut ibv_qp, addr: u64, length: u32, lkey: u32, flags: u32, vlan_tci: *mut u16) -> c_int>,
	pub send_pending_inline_vlan: Option<unsafe extern "C" fn(qp: *mut ibv_qp, addr: *mut c_void, length: u32, flags: u32, vlan_tci: *mut u16) -> c_int>,
	pub send_pending_sg_list_vlan: Option<unsafe extern "C" fn(qp: *mut ibv_qp, sg_list: *mut ibv_sge, num: u32, flags: u32, vlan_tci: *mut u16) -> c_int>,
}

impl Default for ibv_exp_qp_burst_family_v1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
