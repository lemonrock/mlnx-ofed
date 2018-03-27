// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_path_record
{
	pub service_id: u64,
	pub dgid: ibv_gid,
	pub sgid: ibv_gid,
	pub dlid: u16,
	pub slid: u16,
	pub flowlabel_hoplimit: u32,
	pub tclass: u8,
	pub reversible_numpath: u8,
	pub pkey: u16,
	pub qosclass_sl: u16,
	pub mtu: u8,
	pub rate: u8,
	pub packetlifetime: u8,
	pub preference: u8,
	pub reserved: [u8; 6usize],
}

impl Default for ibv_path_record
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_path_record
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_path_record {{ dgid: {:?}, sgid: {:?}, reserved: {:?} }}", self.dgid, self.sgid, self.reserved)
	}
}
