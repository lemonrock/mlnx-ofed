// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct verbs_context
{
	pub query_device_ex: Option<unsafe extern "C" fn(context: *mut ibv_context, input: *const ibv_query_device_ex_input, attr: *mut ibv_device_attr_ex, attr_size: usize) -> c_int>,
	pub _reserved_2: Option<unsafe extern "C" fn() -> c_int>,
	pub destroy_flow: Option<unsafe extern "C" fn(flow: *mut ibv_flow) -> c_int>,
	pub _reserved_1: Option<unsafe extern "C" fn() -> c_int>,
	pub create_flow: Option<unsafe extern "C" fn(qp: *mut ibv_qp, flow_attr: *mut ibv_flow_attr) -> *mut ibv_flow>,
	pub open_qp: Option<unsafe extern "C" fn(context: *mut ibv_context, attr: *mut ibv_qp_open_attr) -> *mut ibv_qp>,
	pub create_qp_ex: Option<unsafe extern "C" fn(context: *mut ibv_context, qp_init_attr_ex: *mut ibv_qp_init_attr_ex) -> *mut ibv_qp>,
	pub get_srq_num: Option<unsafe extern "C" fn(srq: *mut ibv_srq, srq_num: *mut u32) -> c_int>,
	pub create_srq_ex: Option<unsafe extern "C" fn(context: *mut ibv_context, srq_init_attr_ex: *mut ibv_srq_init_attr_ex) -> *mut ibv_srq>,
	pub open_xrcd: Option<unsafe extern "C" fn(context: *mut ibv_context, xrcd_init_attr: *mut ibv_xrcd_init_attr) -> *mut ibv_xrcd>,
	pub close_xrcd: Option<unsafe extern "C" fn(xrcd: *mut ibv_xrcd) -> c_int>,
	pub has_comp_mask: u64,
	pub sz: usize,
	pub context: ibv_context,
}

impl Default for verbs_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for verbs_context
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "verbs_context {{ query_device_ex: {:?}, _reserved_2: {:?}, destroy_flow: {:?}, _reserved_1: {:?}, create_flow: {:?}, open_qp: {:?}, create_qp_ex: {:?}, get_srq_num: {:?}, create_srq_ex: {:?}, open_xrcd: {:?}, close_xrcd: {:?}, context: {:?} }}", self.query_device_ex, self._reserved_2, self.destroy_flow, self._reserved_1, self.create_flow, self.open_qp, self.create_qp_ex, self.get_srq_num, self.create_srq_ex, self.open_xrcd, self.close_xrcd, self.context)
	}
}
