// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_kern_path_rec
{
	pub dgid: [__u8; 16usize],
	pub sgid: [__u8; 16usize],
	pub dlid: __u16,
	pub slid: __u16,
	pub raw_traffic: __u32,
	pub flow_label: __u32,
	pub reversible: __u32,
	pub mtu: __u32,
	pub pkey: __u16,
	pub hop_limit: __u8,
	pub traffic_class: __u8,
	pub numb_path: __u8,
	pub sl: __u8,
	pub mtu_selector: __u8,
	pub rate_selector: __u8,
	pub rate: __u8,
	pub packet_life_time_selector: __u8,
	pub packet_life_time: __u8,
	pub preference: __u8,
}

impl Default for ibv_kern_path_rec
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_kern_path_rec
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_kern_path_rec {{ dgid: {:?}, sgid: {:?} }}", self.dgid, self.sgid)
	}
}
