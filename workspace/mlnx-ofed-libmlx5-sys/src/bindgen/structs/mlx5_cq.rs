// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_cq
{
	pub ibv_cq: ibv_cq,
	pub creation_flags: u32,
	pub pattern: u32,
	pub buf_a: mlx5_buf,
	pub buf_b: mlx5_buf,
	pub active_buf: *mut mlx5_buf,
	pub resize_buf: *mut mlx5_buf,
	pub resize_cqes: c_int,
	pub active_cqes: c_int,
	pub lock: mlx5_lock,
	pub cqn: u32,
	pub cons_index: u32,
	pub wait_index: u32,
	pub wait_count: u32,
	pub dbrec: *mut u32,
	pub arm_sn: c_int,
	pub cqe_sz: c_int,
	pub resize_cqe_sz: c_int,
	pub stall_next_poll: c_int,
	pub stall_enable: c_int,
	pub stall_last_count: u64,
	pub stall_adaptive_enable: c_int,
	pub stall_cycles: c_int,
	pub model_flags: u8,
	pub cqe_comp_max_num: u16,
	pub cq_log_size: u8,
	pub next_decomp_cqe64: mlx5_cqe64,
	pub compressed_rsc: *mut mlx5_resource,
	pub compressed_left: u16,
	pub compressed_wqe_cnt: u16,
	pub compressed_req: u8,
	pub compressed_mp_rq: u8,
	pub mini_arr_idx: u8,
	pub mini_array: [mlx5_mini_cqe8; 8usize],
	pub peer_enabled: c_int,
	pub peer_ctx: *mut ibv_exp_peer_direct_attr,
	pub peer_buf: mlx5_buf,
	pub peer_peek_table: *mut *mut mlx5_peek_entry,
	pub peer_peek_free: *mut mlx5_peek_entry,
}

impl Default for mlx5_cq
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_cq
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_cq {{ buf_a: {:?}, buf_b: {:?}, active_buf: {:?}, resize_buf: {:?}, lock: {:?}, dbrec: {:?}, next_decomp_cqe64: {:?}, compressed_rsc: {:?}, mini_array: {:?}, peer_ctx: {:?}, peer_buf: {:?}, peer_peek_table: {:?}, peer_peek_free: {:?} }}", self.buf_a, self.buf_b, self.active_buf, self.resize_buf, self.lock, self.dbrec, self.next_decomp_cqe64, self.compressed_rsc, self.mini_array, self.peer_ctx, self.peer_buf, self.peer_peek_table, self.peer_peek_free)
	}
}
