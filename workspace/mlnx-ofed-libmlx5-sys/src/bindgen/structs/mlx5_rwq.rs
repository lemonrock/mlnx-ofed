// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_rwq
{
	pub rsc: mlx5_resource,
	pub pattern: u32,
	pub wq: ibv_exp_wq,
	pub buf: mlx5_buf,
	pub buf_size: c_int,
	pub rq: mlx5_wq,
	pub db: *mut u32,
	pub consumed_strides_counter: *mut u32,
	pub mp_rq_stride_size: u16,
	pub mp_rq_strides_in_wqe: u32,
	pub mp_rq_packet_padding: u8,
	pub rq_enable: mlx5_wq_recv_send_enable,
	pub wq_sig: c_int,
	pub model_flags: u8,
}

impl Default for mlx5_rwq
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_rwq
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_rwq {{ rsc: {:?}, buf: {:?}, rq: {:?}, db: {:?}, consumed_strides_counter: {:?}, rq_enable: {:?} }}", self.rsc, self.buf, self.rq, self.db, self.consumed_strides_counter, self.rq_enable)
	}
}
