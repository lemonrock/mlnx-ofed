// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


extern "C"
{
	pub fn rdma_accept(id: *mut rdma_cm_id, conn_param: *mut rdma_conn_param) -> c_int;
	pub fn rdma_ack_cm_event(event: *mut rdma_cm_event) -> c_int;
	pub fn rdma_bind_addr(id: *mut rdma_cm_id, addr: *mut sockaddr) -> c_int;
	pub fn rdma_connect(id: *mut rdma_cm_id, conn_param: *mut rdma_conn_param) -> c_int;
	pub fn rdma_create_ep(id: *mut *mut rdma_cm_id, res: *mut rdma_addrinfo, pd: *mut ibv_pd, qp_init_attr: *mut ibv_qp_init_attr) -> c_int;
	pub fn rdma_create_event_channel() -> *mut rdma_event_channel;
	pub fn rdma_create_id(channel: *mut rdma_event_channel, id: *mut *mut rdma_cm_id, context: *mut c_void, ps: rdma_port_space) -> c_int;
	pub fn rdma_create_qp(id: *mut rdma_cm_id, pd: *mut ibv_pd, qp_init_attr: *mut ibv_qp_init_attr) -> c_int;
	pub fn rdma_create_qp_ex(id: *mut rdma_cm_id, qp_init_attr: *mut ibv_qp_init_attr_ex) -> c_int;
	pub fn rdma_create_qp_exp(id: *mut rdma_cm_id, qp_init_attr: *mut ibv_exp_qp_init_attr) -> c_int;
	pub fn rdma_create_srq(id: *mut rdma_cm_id, pd: *mut ibv_pd, attr: *mut ibv_srq_init_attr) -> c_int;
	pub fn rdma_create_srq_ex(id: *mut rdma_cm_id, attr: *mut ibv_srq_init_attr_ex) -> c_int;
	pub fn rdma_destroy_ep(id: *mut rdma_cm_id);
	pub fn rdma_destroy_event_channel(channel: *mut rdma_event_channel);
	pub fn rdma_destroy_id(id: *mut rdma_cm_id) -> c_int;
	pub fn rdma_destroy_qp(id: *mut rdma_cm_id);
	pub fn rdma_destroy_srq(id: *mut rdma_cm_id);
	pub fn rdma_disconnect(id: *mut rdma_cm_id) -> c_int;
	pub fn rdma_event_str(event: rdma_cm_event_type) -> *const c_char;
	pub fn rdma_free_devices(list: *mut *mut ibv_context);
	pub fn rdma_freeaddrinfo(res: *mut rdma_addrinfo);
	pub fn rdma_get_cm_event(channel: *mut rdma_event_channel, event: *mut *mut rdma_cm_event) -> c_int;
	pub fn rdma_get_devices(num_devices: *mut c_int) -> *mut *mut ibv_context;
	pub fn rdma_get_dst_port(id: *mut rdma_cm_id) -> u16;
	pub fn rdma_get_request(listen: *mut rdma_cm_id, id: *mut *mut rdma_cm_id) -> c_int;
	pub fn rdma_get_src_port(id: *mut rdma_cm_id) -> u16;
	pub fn rdma_getaddrinfo(node: *mut c_char, service: *mut c_char, hints: *mut rdma_addrinfo, res: *mut *mut rdma_addrinfo) -> c_int;
	pub fn rdma_join_multicast(id: *mut rdma_cm_id, addr: *mut sockaddr, context: *mut c_void) -> c_int;
	pub fn rdma_leave_multicast(id: *mut rdma_cm_id, addr: *mut sockaddr) -> c_int;
	pub fn rdma_lib_reset() -> c_int;
	pub fn rdma_listen(id: *mut rdma_cm_id, backlog: c_int) -> c_int;
	pub fn rdma_migrate_id(id: *mut rdma_cm_id, channel: *mut rdma_event_channel) -> c_int;
	pub fn rdma_notify(id: *mut rdma_cm_id, event: ibv_event_type) -> c_int;
	pub fn rdma_reject(id: *mut rdma_cm_id, private_data: *const c_void, private_data_len: u8) -> c_int;
	pub fn rdma_resolve_addr(id: *mut rdma_cm_id, src_addr: *mut sockaddr, dst_addr: *mut sockaddr, timeout_ms: c_int) -> c_int;
	pub fn rdma_resolve_route(id: *mut rdma_cm_id, timeout_ms: c_int) -> c_int;
	pub fn rdma_set_option(id: *mut rdma_cm_id, level: c_int, optname: c_int, optval: *mut c_void, optlen: usize) -> c_int;
}
