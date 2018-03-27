// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_srq
{
	pub rsc: mlx5_resource,
	pub vsrq: verbs_srq,
	pub buf: mlx5_buf,
	pub lock: mlx5_spinlock,
	pub wrid: *mut u64,
	pub srqn: u32,
	pub max: c_int,
	pub max_gs: c_int,
	pub wqe_shift: c_int,
	pub head: c_int,
	pub tail: c_int,
	pub db: *mut u32,
	pub counter: u16,
	pub wq_sig: c_int,
	pub ibv_srq_legacy: *mut ibv_srq_legacy,
	pub is_xsrq: c_int,
	pub cmd_qp: *mut mlx5_qp,
	pub tm_list: *mut mlx5_tag_entry,
	pub tm_head: *mut mlx5_tag_entry,
	pub tm_tail: *mut mlx5_tag_entry,
	pub op: *mut mlx5_srq_op,
	pub op_head: c_int,
	pub op_tail: c_int,
	pub unexp_in: c_int,
	pub unexp_out: c_int,
}

impl Default for mlx5_srq
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_srq
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_srq {{ rsc: {:?}, buf: {:?}, lock: {:?}, wrid: {:?}, db: {:?}, ibv_srq_legacy: {:?}, cmd_qp: {:?}, tm_list: {:?}, tm_head: {:?}, tm_tail: {:?}, op: {:?} }}", self.rsc, self.buf, self.lock, self.wrid, self.db, self.ibv_srq_legacy, self.cmd_qp, self.tm_list, self.tm_head, self.tm_tail, self.op)
	}
}
