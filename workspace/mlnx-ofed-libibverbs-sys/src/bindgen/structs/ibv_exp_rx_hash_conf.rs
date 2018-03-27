// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_exp_rx_hash_conf
{
	pub rx_hash_function: u8,
	pub rx_hash_key_len: u8,
	pub rx_hash_key: *mut u8,
	pub rx_hash_fields_mask: u64,
	pub rwq_ind_tbl: *mut ibv_exp_rwq_ind_table,
}

impl Default for ibv_exp_rx_hash_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_exp_rx_hash_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_exp_rx_hash_conf {{ rx_hash_key: {:?}, rwq_ind_tbl: {:?} }}", self.rx_hash_key, self.rwq_ind_tbl)
	}
}
