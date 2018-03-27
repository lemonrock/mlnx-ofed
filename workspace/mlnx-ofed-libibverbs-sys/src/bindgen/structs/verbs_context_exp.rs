// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct verbs_context_exp
{
	pub exp_alloc_dm: Option<unsafe extern "C" fn(context: *mut ibv_context, attr: *mut ibv_exp_alloc_dm_attr) -> *mut ibv_exp_dm>,
	pub exp_free_dm: Option<unsafe extern "C" fn(dm: *mut ibv_exp_dm) -> c_int>,
	pub exp_memcpy_dm: Option<unsafe extern "C" fn(dm: *mut ibv_exp_dm, attr: *mut ibv_exp_memcpy_dm_attr) -> c_int>,
	pub exp_post_srq_ops: Option<unsafe extern "C" fn(srq: *mut ibv_srq, op: *mut ibv_exp_ops_wr, bad_op: *mut *mut ibv_exp_ops_wr) -> c_int>,
	pub exp_create_srq: Option<unsafe extern "C" fn(context: *mut ibv_context, attr: *mut ibv_exp_create_srq_attr) -> *mut ibv_srq>,
	pub drv_exp_set_context_attr: Option<unsafe extern "C" fn(context: *mut ibv_context, attr: *mut ibv_exp_open_device_attr) -> c_int>,
	pub drv_exp_ibv_create_kah: Option<unsafe extern "C" fn(pd: *mut ibv_pd, attr_exp: *mut ibv_exp_ah_attr) -> *mut ibv_ah>,
	pub exp_peer_peek_cq: Option<unsafe extern "C" fn(ibcq: *mut ibv_cq, peek_ctx: *mut ibv_exp_peer_peek) -> c_int>,
	pub exp_peer_abort_peek_cq: Option<unsafe extern "C" fn(ibcq: *mut ibv_cq, ack_ctx: *mut ibv_exp_peer_abort_peek) -> c_int>,
	pub exp_peer_commit_qp: Option<unsafe extern "C" fn(qp: *mut ibv_qp, peer: *mut ibv_exp_peer_commit) -> c_int>,
	pub exp_rollback_send: Option<unsafe extern "C" fn(qp: *mut ibv_qp, rollback: *mut ibv_exp_rollback_ctx) -> c_int>,
	pub ec_update_sync: Option<unsafe extern "C" fn(calc: *mut ibv_exp_ec_calc, ec_mem: *mut ibv_exp_ec_mem, data_updates: *mut u8, code_updates: *mut u8) -> c_int>,
	pub ec_update_async: Option<unsafe extern "C" fn(calc: *mut ibv_exp_ec_calc, ec_mem: *mut ibv_exp_ec_mem, data_updates: *mut u8, code_updates: *mut u8, ec_comp: *mut ibv_exp_ec_comp) -> c_int>,
	pub alloc_ec_calc: Option<unsafe extern "C" fn(pd: *mut ibv_pd, attr: *mut ibv_exp_ec_calc_init_attr) -> *mut ibv_exp_ec_calc>,
	pub dealloc_ec_calc: Option<unsafe extern "C" fn(calc: *mut ibv_exp_ec_calc)>,
	pub ec_encode_async: Option<unsafe extern "C" fn(calc: *mut ibv_exp_ec_calc, ec_mem: *mut ibv_exp_ec_mem, ec_comp: *mut ibv_exp_ec_comp) -> c_int>,
	pub ec_encode_sync: Option<unsafe extern "C" fn(calc: *mut ibv_exp_ec_calc, ec_mem: *mut ibv_exp_ec_mem) -> c_int>,
	pub ec_decode_async: Option<unsafe extern "C" fn(calc: *mut ibv_exp_ec_calc, ec_mem: *mut ibv_exp_ec_mem, erasures: *mut u8, decode_matrix: *mut u8, ec_comp: *mut ibv_exp_ec_comp) -> c_int>,
	pub ec_decode_sync: Option<unsafe extern "C" fn(calc: *mut ibv_exp_ec_calc, ec_mem: *mut ibv_exp_ec_mem, erasures: *mut u8, decode_matrix: *mut u8) -> c_int>,
	pub ec_poll: Option<unsafe extern "C" fn(calc: *mut ibv_exp_ec_calc, n: c_int) -> c_int>,
	pub ec_encode_send: Option<unsafe extern "C" fn(calc: *mut ibv_exp_ec_calc, ec_mem: *mut ibv_exp_ec_mem, data_stripes: *mut ibv_exp_ec_stripe, code_stripes: *mut ibv_exp_ec_stripe) -> c_int>,
	pub exp_query_gid_attr: Option<unsafe extern "C" fn(context: *mut ibv_context, port_num: u8, index: c_uint, attr: *mut ibv_exp_gid_attr) -> c_int>,
	pub exp_destroy_rwq_ind_table: Option<unsafe extern "C" fn(rwq_ind_table: *mut ibv_exp_rwq_ind_table) -> c_int>,
	pub exp_create_rwq_ind_table: Option<unsafe extern "C" fn(context: *mut ibv_context, init_attr: *mut ibv_exp_rwq_ind_table_init_attr) -> *mut ibv_exp_rwq_ind_table>,
	pub exp_destroy_wq: Option<unsafe extern "C" fn(wq: *mut ibv_exp_wq) -> c_int>,
	pub exp_modify_wq: Option<unsafe extern "C" fn(wq: *mut ibv_exp_wq, wq_attr: *mut ibv_exp_wq_attr) -> c_int>,
	pub exp_create_wq: Option<unsafe extern "C" fn(context: *mut ibv_context, wq_init_attr: *mut ibv_exp_wq_init_attr) -> *mut ibv_exp_wq>,
	pub drv_exp_poll_dc_info: Option<unsafe extern "C" fn(context: *mut ibv_context, ents: *mut ibv_exp_dc_info_ent, nent: c_int, port: c_int) -> c_int>,
	pub exp_query_intf: Option<unsafe extern "C" fn(context: *mut ibv_context, params: *mut ibv_exp_query_intf_params, status: *mut ibv_exp_query_intf_status) -> *mut c_void>,
	pub exp_release_intf: Option<unsafe extern "C" fn(context: *mut ibv_context, intf: *mut c_void, params: *mut ibv_exp_release_intf_params) -> c_int>,
	pub exp_create_res_domain: Option<unsafe extern "C" fn(context: *mut ibv_context, attr: *mut ibv_exp_res_domain_init_attr) -> *mut ibv_exp_res_domain>,
	pub exp_destroy_res_domain: Option<unsafe extern "C" fn(context: *mut ibv_context, res_dom: *mut ibv_exp_res_domain, attr: *mut ibv_exp_destroy_res_domain_attr) -> c_int>,
	pub lib_exp_use_priv_env: Option<unsafe extern "C" fn(context: *mut ibv_context) -> c_int>,
	pub lib_exp_setenv: Option<unsafe extern "C" fn(context: *mut ibv_context, name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int>,
	pub venv: *mut verbs_environment,
	pub drv_exp_dereg_mr: Option<unsafe extern "C" fn(mr: *mut ibv_mr, out: *mut ibv_exp_dereg_out) -> c_int>,
	pub ABI_placeholder2: Option<unsafe extern "C" fn()>,
	pub ABI_placeholder1: Option<unsafe extern "C" fn()>,
	pub drv_exp_prefetch_mr: Option<unsafe extern "C" fn(mr: *mut ibv_mr, attr: *mut ibv_exp_prefetch_attr) -> c_int>,
	pub lib_exp_prefetch_mr: Option<unsafe extern "C" fn(mr: *mut ibv_mr, attr: *mut ibv_exp_prefetch_attr) -> c_int>,
	pub drv_exp_alloc_mkey_list_memory: Option<unsafe extern "C" fn(attr: *mut ibv_exp_mkey_list_container_attr) -> *mut ibv_exp_mkey_list_container>,
	pub lib_exp_alloc_mkey_list_memory: Option<unsafe extern "C" fn(attr: *mut ibv_exp_mkey_list_container_attr) -> *mut ibv_exp_mkey_list_container>,
	pub drv_exp_dealloc_mkey_list_memory: Option<unsafe extern "C" fn(mem: *mut ibv_exp_mkey_list_container) -> c_int>,
	pub lib_exp_dealloc_mkey_list_memory: Option<unsafe extern "C" fn(mem: *mut ibv_exp_mkey_list_container) -> c_int>,
	pub drv_exp_query_mkey: Option<unsafe extern "C" fn(mr: *mut ibv_mr, mkey_attr: *mut ibv_exp_mkey_attr) -> c_int>,
	pub lib_exp_query_mkey: Option<unsafe extern "C" fn(mr: *mut ibv_mr, mkey_attr: *mut ibv_exp_mkey_attr) -> c_int>,
	pub drv_exp_create_mr: Option<unsafe extern "C" fn(in_: *mut ibv_exp_create_mr_in) -> *mut ibv_mr>,
	pub lib_exp_create_mr: Option<unsafe extern "C" fn(in_: *mut ibv_exp_create_mr_in) -> *mut ibv_mr>,
	pub drv_exp_arm_dct: Option<unsafe extern "C" fn(dct: *mut ibv_exp_dct, attr: *mut ibv_exp_arm_attr) -> c_int>,
	pub lib_exp_arm_dct: Option<unsafe extern "C" fn(dct: *mut ibv_exp_dct, attr: *mut ibv_exp_arm_attr) -> c_int>,
	pub drv_exp_bind_mw: Option<unsafe extern "C" fn(mw_bind: *mut ibv_exp_mw_bind) -> c_int>,
	pub lib_exp_bind_mw: Option<unsafe extern "C" fn(mw_bind: *mut ibv_exp_mw_bind) -> c_int>,
	pub drv_exp_post_send: Option<unsafe extern "C" fn(qp: *mut ibv_qp, wr: *mut ibv_exp_send_wr, bad_wr: *mut *mut ibv_exp_send_wr) -> c_int>,
	pub drv_exp_reg_mr: Option<unsafe extern "C" fn(in_: *mut ibv_exp_reg_mr_in) -> *mut ibv_mr>,
	pub lib_exp_reg_mr: Option<unsafe extern "C" fn(in_: *mut ibv_exp_reg_mr_in) -> *mut ibv_mr>,
	pub drv_exp_ibv_create_ah: Option<unsafe extern "C" fn(pd: *mut ibv_pd, attr_exp: *mut ibv_exp_ah_attr) -> *mut ibv_ah>,
	pub drv_exp_query_values: Option<unsafe extern "C" fn(context: *mut ibv_context, q_values: c_int, values: *mut ibv_exp_values) -> c_int>,
	pub exp_create_cq: Option<unsafe extern "C" fn(context: *mut ibv_context, cqe: c_int, channel: *mut ibv_comp_channel, comp_vector: c_int, attr: *mut ibv_exp_cq_init_attr) -> *mut ibv_cq>,
	pub drv_exp_ibv_poll_cq: Option<unsafe extern "C" fn(ibcq: *mut ibv_cq, num_entries: c_int, wc: *mut ibv_exp_wc, wc_size: u32) -> c_int>,
	pub drv_exp_get_legacy_xrc: Option<unsafe extern "C" fn(ibv_srq: *mut ibv_srq) -> *mut c_void>,
	pub drv_exp_set_legacy_xrc: Option<unsafe extern "C" fn(ibv_srq: *mut ibv_srq, legacy_xrc: *mut c_void)>,
	pub drv_exp_ibv_reg_shared_mr: Option<unsafe extern "C" fn(in_: *mut ibv_exp_reg_shared_mr_in) -> *mut ibv_mr>,
	pub lib_exp_ibv_reg_shared_mr: Option<unsafe extern "C" fn(in_: *mut ibv_exp_reg_shared_mr_in) -> *mut ibv_mr>,
	pub drv_exp_modify_qp: Option<unsafe extern "C" fn(qp: *mut ibv_qp, attr: *mut ibv_exp_qp_attr, exp_attr_mask: u64) -> c_int>,
	pub lib_exp_modify_qp: Option<unsafe extern "C" fn(qp: *mut ibv_qp, attr: *mut ibv_exp_qp_attr, exp_attr_mask: u64) -> c_int>,
	pub drv_exp_post_task: Option<unsafe extern "C" fn(context: *mut ibv_context, task: *mut ibv_exp_task, bad_task: *mut *mut ibv_exp_task) -> c_int>,
	pub lib_exp_post_task: Option<unsafe extern "C" fn(context: *mut ibv_context, task: *mut ibv_exp_task, bad_task: *mut *mut ibv_exp_task) -> c_int>,
	pub drv_exp_modify_cq: Option<unsafe extern "C" fn(cq: *mut ibv_cq, attr: *mut ibv_exp_cq_attr, attr_mask: c_int) -> c_int>,
	pub lib_exp_modify_cq: Option<unsafe extern "C" fn(cq: *mut ibv_cq, attr: *mut ibv_exp_cq_attr, attr_mask: c_int) -> c_int>,
	pub drv_exp_ibv_destroy_flow: Option<unsafe extern "C" fn(flow: *mut ibv_exp_flow) -> c_int>,
	pub lib_exp_ibv_destroy_flow: Option<unsafe extern "C" fn(flow: *mut ibv_exp_flow) -> c_int>,
	pub drv_exp_ibv_create_flow: Option<unsafe extern "C" fn(qp: *mut ibv_qp, flow_attr: *mut ibv_exp_flow_attr) -> *mut ibv_exp_flow>,
	pub lib_exp_ibv_create_flow: Option<unsafe extern "C" fn(qp: *mut ibv_qp, flow_attr: *mut ibv_exp_flow_attr) -> *mut ibv_exp_flow>,
	pub drv_exp_query_port: Option<unsafe extern "C" fn(context: *mut ibv_context, port_num: u8, port_attr: *mut ibv_exp_port_attr) -> c_int>,
	pub lib_exp_query_port: Option<unsafe extern "C" fn(context: *mut ibv_context, port_num: u8, port_attr: *mut ibv_exp_port_attr) -> c_int>,
	pub create_dct: Option<unsafe extern "C" fn(context: *mut ibv_context, attr: *mut ibv_exp_dct_init_attr) -> *mut ibv_exp_dct>,
	pub destroy_dct: Option<unsafe extern "C" fn(dct: *mut ibv_exp_dct) -> c_int>,
	pub query_dct: Option<unsafe extern "C" fn(dct: *mut ibv_exp_dct, attr: *mut ibv_exp_dct_attr) -> c_int>,
	pub drv_exp_query_device: Option<unsafe extern "C" fn(context: *mut ibv_context, attr: *mut ibv_exp_device_attr) -> c_int>,
	pub lib_exp_query_device: Option<unsafe extern "C" fn(context: *mut ibv_context, attr: *mut ibv_exp_device_attr) -> c_int>,
	pub drv_exp_create_qp: Option<unsafe extern "C" fn(context: *mut ibv_context, init_attr: *mut ibv_exp_qp_init_attr) -> *mut ibv_qp>,
	pub lib_exp_create_qp: Option<unsafe extern "C" fn(context: *mut ibv_context, init_attr: *mut ibv_exp_qp_init_attr) -> *mut ibv_qp>,
	pub sz: usize,
}

impl Default for verbs_context_exp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for verbs_context_exp
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"verbs_context_exp {{ exp_alloc_dm: {:?}, exp_free_dm: {:?}, exp_memcpy_dm: {:?}, exp_post_srq_ops: {:?}, exp_create_srq: {:?}, drv_exp_set_context_attr: {:?}, drv_exp_ibv_create_kah: {:?}, exp_peer_peek_cq: {:?}, exp_peer_abort_peek_cq: {:?}, exp_peer_commit_qp: {:?}, exp_rollback_send: {:?}, ec_update_sync: {:?}, ec_update_async: {:?}, alloc_ec_calc: {:?}, dealloc_ec_calc: {:?}, ec_encode_async: {:?}, ec_encode_sync: {:?}, ec_decode_async: {:?}, ec_decode_sync: {:?}, ec_poll: {:?}, ec_encode_send: {:?}, exp_query_gid_attr: {:?}, exp_destroy_rwq_ind_table: {:?}, exp_create_rwq_ind_table: {:?}, exp_destroy_wq: {:?}, exp_modify_wq: {:?}, exp_create_wq: {:?}, drv_exp_poll_dc_info: {:?}, exp_query_intf: {:?}, exp_release_intf: {:?}, exp_create_res_domain: {:?}, exp_destroy_res_domain: {:?}, lib_exp_use_priv_env: {:?}, lib_exp_setenv: {:?}, venv: {:?}, drv_exp_dereg_mr: {:?}, ABI_placeholder2: {:?}, ABI_placeholder1: {:?}, drv_exp_prefetch_mr: {:?}, lib_exp_prefetch_mr: {:?}, drv_exp_alloc_mkey_list_memory: {:?}, lib_exp_alloc_mkey_list_memory: {:?}, drv_exp_dealloc_mkey_list_memory: {:?}, lib_exp_dealloc_mkey_list_memory: {:?}, drv_exp_query_mkey: {:?}, lib_exp_query_mkey: {:?}, drv_exp_create_mr: {:?}, lib_exp_create_mr: {:?}, drv_exp_arm_dct: {:?}, lib_exp_arm_dct: {:?}, drv_exp_bind_mw: {:?}, lib_exp_bind_mw: {:?}, drv_exp_post_send: {:?}, drv_exp_reg_mr: {:?}, lib_exp_reg_mr: {:?}, drv_exp_ibv_create_ah: {:?}, drv_exp_query_values: {:?}, exp_create_cq: {:?}, drv_exp_ibv_poll_cq: {:?}, drv_exp_get_legacy_xrc: {:?}, drv_exp_set_legacy_xrc: {:?}, drv_exp_ibv_reg_shared_mr: {:?}, lib_exp_ibv_reg_shared_mr: {:?}, drv_exp_modify_qp: {:?}, lib_exp_modify_qp: {:?}, drv_exp_post_task: {:?}, lib_exp_post_task: {:?}, drv_exp_modify_cq: {:?}, lib_exp_modify_cq: {:?}, drv_exp_ibv_destroy_flow: {:?}, lib_exp_ibv_destroy_flow: {:?}, drv_exp_ibv_create_flow: {:?}, lib_exp_ibv_create_flow: {:?}, drv_exp_query_port: {:?}, lib_exp_query_port: {:?}, create_dct: {:?}, destroy_dct: {:?}, query_dct: {:?}, drv_exp_query_device: {:?}, lib_exp_query_device: {:?}, drv_exp_create_qp: {:?}, lib_exp_create_qp: {:?} }}",
			self.exp_alloc_dm, self.exp_free_dm, self.exp_memcpy_dm, self.exp_post_srq_ops, self.exp_create_srq, self.drv_exp_set_context_attr, self.drv_exp_ibv_create_kah, self.exp_peer_peek_cq, self.exp_peer_abort_peek_cq, self.exp_peer_commit_qp, self.exp_rollback_send, self.ec_update_sync, self.ec_update_async, self.alloc_ec_calc, self.dealloc_ec_calc, self.ec_encode_async, self.ec_encode_sync, self.ec_decode_async, self.ec_decode_sync, self.ec_poll, self.ec_encode_send, self.exp_query_gid_attr, self.exp_destroy_rwq_ind_table, self.exp_create_rwq_ind_table, self.exp_destroy_wq, self.exp_modify_wq, self.exp_create_wq, self.drv_exp_poll_dc_info, self.exp_query_intf, self.exp_release_intf, self.exp_create_res_domain, self.exp_destroy_res_domain, self.lib_exp_use_priv_env, self.lib_exp_setenv, self.venv, self.drv_exp_dereg_mr, self.ABI_placeholder2, self.ABI_placeholder1, self.drv_exp_prefetch_mr, self.lib_exp_prefetch_mr, self.drv_exp_alloc_mkey_list_memory, self.lib_exp_alloc_mkey_list_memory, self.drv_exp_dealloc_mkey_list_memory, self.lib_exp_dealloc_mkey_list_memory, self.drv_exp_query_mkey, self.lib_exp_query_mkey, self.drv_exp_create_mr, self.lib_exp_create_mr, self.drv_exp_arm_dct, self.lib_exp_arm_dct, self.drv_exp_bind_mw, self.lib_exp_bind_mw, self.drv_exp_post_send, self.drv_exp_reg_mr, self.lib_exp_reg_mr, self.drv_exp_ibv_create_ah, self.drv_exp_query_values, self.exp_create_cq, self.drv_exp_ibv_poll_cq, self.drv_exp_get_legacy_xrc, self.drv_exp_set_legacy_xrc, self.drv_exp_ibv_reg_shared_mr, self.lib_exp_ibv_reg_shared_mr, self.drv_exp_modify_qp, self.lib_exp_modify_qp, self.drv_exp_post_task, self.lib_exp_post_task, self.drv_exp_modify_cq, self.lib_exp_modify_cq, self.drv_exp_ibv_destroy_flow, self.lib_exp_ibv_destroy_flow, self.drv_exp_ibv_create_flow, self.lib_exp_ibv_create_flow, self.drv_exp_query_port, self.lib_exp_query_port, self.create_dct, self.destroy_dct, self.query_dct, self.drv_exp_query_device, self.lib_exp_query_device, self.drv_exp_create_qp, self.lib_exp_create_qp
		)
	}
}
