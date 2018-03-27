// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_qp
{
	pub rsc: mlx5_resource,
	pub verbs_qp: verbs_qp,
	pub buf: mlx5_buf,
	pub buf_size: c_int,
	pub sq_buf: mlx5_buf,
	pub sq_buf_size: c_int,
	pub sq_signal_bits: u8,
	pub umr_en: c_int,
	pub rq: mlx5_wq,
	pub sq: mlx5_wq,
	pub gen_data: general_data_hot,
	pub mpw: mpw_data,
	pub data_seg: data_seg_data,
	pub ctrl_seg: ctrl_seg_data,
	pub link_layer: u8,
	pub gen_data_warm: general_data_warm,
	pub enable_atomics: c_int,
	pub odp_data: odp_data,
	pub max_atomic_arg: u32,
	pub max_inl_send_klms: u32,
	pub rq_enable: mlx5_wq_recv_send_enable,
	pub sq_enable: mlx5_wq_recv_send_enable,
	pub rx_qp: c_int,
	pub peer_enabled: c_int,
	pub peer_ctx: *mut ibv_exp_peer_direct_attr,
	pub peer_ctrl_seg: *mut c_void,
	pub peer_scur_post: u32,
	pub peer_va_ids: [u64; 2usize],
	pub peer_db_buf: *mut ibv_exp_peer_buf,
	pub max_tso_header: u32,
	pub flags: u32,
}

impl Default for mlx5_qp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_qp
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_qp {{ rsc: {:?}, buf: {:?}, sq_buf: {:?}, rq: {:?}, sq: {:?}, gen_data: {:?}, mpw: {:?}, data_seg: {:?}, ctrl_seg: {:?}, gen_data_warm: {:?}, odp_data: {:?}, rq_enable: {:?}, sq_enable: {:?}, peer_ctx: {:?}, peer_ctrl_seg: {:?}, peer_va_ids: {:?}, peer_db_buf: {:?} }}", self.rsc, self.buf, self.sq_buf, self.rq, self.sq, self.gen_data, self.mpw, self.data_seg, self.ctrl_seg, self.gen_data_warm, self.odp_data, self.rq_enable, self.sq_enable, self.peer_ctx, self.peer_ctrl_seg, self.peer_va_ids, self.peer_db_buf)
	}
}
