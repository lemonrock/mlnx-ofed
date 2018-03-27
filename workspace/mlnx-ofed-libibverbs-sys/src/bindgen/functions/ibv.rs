// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ibv_ack_async_event"] pub fn ibv_ack_async_event(event: *mut ibv_async_event);
	#[link_name = "\u{1}_ibv_ack_cq_events"] pub fn ibv_ack_cq_events(cq: *mut ibv_cq, nevents: c_uint);
	#[link_name = "\u{1}_ibv_alloc_pd"] pub fn ibv_alloc_pd(context: *mut ibv_context) -> *mut ibv_pd;
	#[link_name = "\u{1}_ibv_attach_mcast"] pub fn ibv_attach_mcast(qp: *mut ibv_qp, gid: *const ibv_gid, lid: u16) -> c_int;
	#[link_name = "\u{1}_ibv_close_device"] pub fn ibv_close_device(context: *mut ibv_context) -> c_int;
	#[link_name = "\u{1}_ibv_close_xrc_domain"] pub fn ibv_close_xrc_domain(d: *mut ibv_xrc_domain) -> c_int;
	#[link_name = "\u{1}_ibv_create_ah"] pub fn ibv_create_ah(pd: *mut ibv_pd, attr: *mut ibv_ah_attr) -> *mut ibv_ah;
	#[link_name = "\u{1}_ibv_create_ah_from_wc"] pub fn ibv_create_ah_from_wc(pd: *mut ibv_pd, wc: *mut ibv_wc, grh: *mut ibv_grh, port_num: u8) -> *mut ibv_ah;
	#[link_name = "\u{1}_ibv_create_comp_channel"] pub fn ibv_create_comp_channel(context: *mut ibv_context) -> *mut ibv_comp_channel;
	#[link_name = "\u{1}_ibv_create_cq"] pub fn ibv_create_cq(context: *mut ibv_context, cqe: c_int, cq_context: *mut c_void, channel: *mut ibv_comp_channel, comp_vector: c_int) -> *mut ibv_cq;
	#[link_name = "\u{1}_ibv_create_qp"] pub fn ibv_create_qp(pd: *mut ibv_pd, qp_init_attr: *mut ibv_qp_init_attr) -> *mut ibv_qp;
	#[link_name = "\u{1}_ibv_create_srq"] pub fn ibv_create_srq(pd: *mut ibv_pd, srq_init_attr: *mut ibv_srq_init_attr) -> *mut ibv_srq;
	#[link_name = "\u{1}_ibv_create_xrc_rcv_qp"] pub fn ibv_create_xrc_rcv_qp(init_attr: *mut ibv_qp_init_attr, xrc_rcv_qpn: *mut u32) -> c_int;
	#[link_name = "\u{1}_ibv_create_xrc_srq"] pub fn ibv_create_xrc_srq(pd: *mut ibv_pd, xrc_domain: *mut ibv_xrc_domain, xrc_cq: *mut ibv_cq, srq_init_attr: *mut ibv_srq_init_attr) -> *mut ibv_srq;
	#[link_name = "\u{1}_ibv_dealloc_pd"] pub fn ibv_dealloc_pd(pd: *mut ibv_pd) -> c_int;
	#[link_name = "\u{1}_ibv_dereg_mr"] pub fn ibv_dereg_mr(mr: *mut ibv_mr) -> c_int;
	#[link_name = "\u{1}_ibv_destroy_ah"] pub fn ibv_destroy_ah(ah: *mut ibv_ah) -> c_int;
	#[link_name = "\u{1}_ibv_destroy_comp_channel"] pub fn ibv_destroy_comp_channel(channel: *mut ibv_comp_channel) -> c_int;
	#[link_name = "\u{1}_ibv_destroy_cq"] pub fn ibv_destroy_cq(cq: *mut ibv_cq) -> c_int;
	#[link_name = "\u{1}_ibv_destroy_qp"] pub fn ibv_destroy_qp(qp: *mut ibv_qp) -> c_int;
	#[link_name = "\u{1}_ibv_destroy_srq"] pub fn ibv_destroy_srq(srq: *mut ibv_srq) -> c_int;
	#[link_name = "\u{1}_ibv_detach_mcast"] pub fn ibv_detach_mcast(qp: *mut ibv_qp, gid: *const ibv_gid, lid: u16) -> c_int;
	#[link_name = "\u{1}_ibv_event_type_str"] pub fn ibv_event_type_str(event: ibv_event_type) -> *const c_char;
	#[link_name = "\u{1}_ibv_exp_get_device_list"] pub fn ibv_exp_get_device_list(num_devices: *mut c_int) -> *mut *mut ibv_device;
	#[link_name = "\u{1}_ibv_fork_init"] pub fn ibv_fork_init() -> c_int;
	#[link_name = "\u{1}_ibv_free_device_list"] pub fn ibv_free_device_list(list: *mut *mut ibv_device);
	#[link_name = "\u{1}_ibv_get_async_event"] pub fn ibv_get_async_event(context: *mut ibv_context, event: *mut ibv_async_event) -> c_int;
	#[link_name = "\u{1}_ibv_get_cq_event"] pub fn ibv_get_cq_event(channel: *mut ibv_comp_channel, cq: *mut *mut ibv_cq, cq_context: *mut *mut c_void) -> c_int;
	#[link_name = "\u{1}_ibv_get_device_guid"] pub fn ibv_get_device_guid(device: *mut ibv_device) -> u64;
	#[link_name = "\u{1}_ibv_get_device_list"] pub fn ibv_get_device_list(num_devices: *mut c_int) -> *mut *mut ibv_device;
	#[link_name = "\u{1}_ibv_get_device_name"] pub fn ibv_get_device_name(device: *mut ibv_device) -> *const c_char;
	#[link_name = "\u{1}_ibv_init_ah_from_wc"] pub fn ibv_init_ah_from_wc(context: *mut ibv_context, port_num: u8, wc: *mut ibv_wc, grh: *mut ibv_grh, ah_attr: *mut ibv_ah_attr) -> c_int;
	#[link_name = "\u{1}_ibv_modify_qp"] pub fn ibv_modify_qp(qp: *mut ibv_qp, attr: *mut ibv_qp_attr, attr_mask: c_int) -> c_int;
	#[link_name = "\u{1}_ibv_modify_srq"] pub fn ibv_modify_srq(srq: *mut ibv_srq, srq_attr: *mut ibv_srq_attr, srq_attr_mask: c_int) -> c_int;
	#[link_name = "\u{1}_ibv_modify_xrc_rcv_qp"] pub fn ibv_modify_xrc_rcv_qp(xrc_domain: *mut ibv_xrc_domain, xrc_qp_num: u32, attr: *mut ibv_qp_attr, attr_mask: c_int) -> c_int;
	#[link_name = "\u{1}_ibv_node_type_str"] pub fn ibv_node_type_str(node_type: ibv_node_type) -> *const c_char;
	#[link_name = "\u{1}_ibv_open_device"] pub fn ibv_open_device(device: *mut ibv_device) -> *mut ibv_context;
	#[link_name = "\u{1}_ibv_open_xrc_domain"] pub fn ibv_open_xrc_domain(context: *mut ibv_context, fd: c_int, oflag: c_int) -> *mut ibv_xrc_domain;
	#[link_name = "\u{1}_ibv_port_state_str"] pub fn ibv_port_state_str(port_state: ibv_port_state) -> *const c_char;
	#[link_name = "\u{1}_ibv_query_device"] pub fn ibv_query_device(context: *mut ibv_context, device_attr: *mut ibv_device_attr) -> c_int;
	#[link_name = "\u{1}_ibv_query_gid"] pub fn ibv_query_gid(context: *mut ibv_context, port_num: u8, index: c_int, gid: *mut ibv_gid) -> c_int;
	#[link_name = "\u{1}_ibv_query_pkey"] pub fn ibv_query_pkey(context: *mut ibv_context, port_num: u8, index: c_int, pkey: *mut u16) -> c_int;
	#[link_name = "\u{1}_ibv_query_port"] pub fn ibv_query_port(context: *mut ibv_context, port_num: u8, port_attr: *mut ibv_port_attr) -> c_int;
	#[link_name = "\u{1}_ibv_query_qp"] pub fn ibv_query_qp(qp: *mut ibv_qp, attr: *mut ibv_qp_attr, attr_mask: c_int, init_attr: *mut ibv_qp_init_attr) -> c_int;
	#[link_name = "\u{1}_ibv_query_srq"] pub fn ibv_query_srq(srq: *mut ibv_srq, srq_attr: *mut ibv_srq_attr) -> c_int;
	#[link_name = "\u{1}_ibv_query_xrc_rcv_qp"] pub fn ibv_query_xrc_rcv_qp(xrc_domain: *mut ibv_xrc_domain, xrc_qp_num: u32, attr: *mut ibv_qp_attr, attr_mask: c_int, init_attr: *mut ibv_qp_init_attr) -> c_int;
	#[link_name = "\u{1}_ibv_rate_to_mbps"] pub fn ibv_rate_to_mbps(rate: ibv_rate) -> c_int;
	#[link_name = "\u{1}_ibv_rate_to_mult"] pub fn ibv_rate_to_mult(rate: ibv_rate) -> c_int;
	#[link_name = "\u{1}_ibv_reg_mr"] pub fn ibv_reg_mr(pd: *mut ibv_pd, addr: *mut c_void, length: usize, access: c_int) -> *mut ibv_mr;
	#[link_name = "\u{1}_ibv_reg_xrc_rcv_qp"] pub fn ibv_reg_xrc_rcv_qp(xrc_domain: *mut ibv_xrc_domain, xrc_qp_num: u32) -> c_int;
	#[link_name = "\u{1}_ibv_rereg_mr"] pub fn ibv_rereg_mr(mr: *mut ibv_mr, flags: c_int, pd: *mut ibv_pd, addr: *mut c_void, length: usize, access: c_int) -> c_int;
	#[link_name = "\u{1}_ibv_resize_cq"] pub fn ibv_resize_cq(cq: *mut ibv_cq, cqe: c_int) -> c_int;
	#[link_name = "\u{1}_ibv_unreg_xrc_rcv_qp"] pub fn ibv_unreg_xrc_rcv_qp(xrc_domain: *mut ibv_xrc_domain, xrc_qp_num: u32) -> c_int;
	#[link_name = "\u{1}_ibv_wc_status_str"] pub fn ibv_wc_status_str(status: ibv_wc_status) -> *const c_char;
}
