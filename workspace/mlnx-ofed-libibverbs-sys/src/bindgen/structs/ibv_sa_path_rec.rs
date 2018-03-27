// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_sa_path_rec
{
	pub dgid: ibv_gid,
	pub sgid: ibv_gid,
	pub dlid: u16,
	pub slid: u16,
	pub raw_traffic: c_int,
	pub flow_label: u32,
	pub hop_limit: u8,
	pub traffic_class: u8,
	pub reversible: c_int,
	pub numb_path: u8,
	pub pkey: u16,
	pub sl: u8,
	pub mtu_selector: u8,
	pub mtu: u8,
	pub rate_selector: u8,
	pub rate: u8,
	pub packet_life_time_selector: u8,
	pub packet_life_time: u8,
	pub preference: u8,
}

impl Default for ibv_sa_path_rec
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_sa_path_rec
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_sa_path_rec {{ dgid: {:?}, sgid: {:?} }}", self.dgid, self.sgid)
	}
}
