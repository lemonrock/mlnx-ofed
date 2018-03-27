// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
pub struct mlx5_cqe64__bindgen_ty_1__bindgen_ty_1
{
	pub rsvd0: [u8; 2usize],
	pub wqe_id: u16,
	pub rsvd4: [u8; 8usize],
	pub rx_hash_res: u32,
	pub rx_hash_type: u8,
	pub ml_path: u8,
	pub rsvd20: [u8; 2usize],
	pub checksum: u16,
	pub slid: u16,
	pub flags_rqpn: u32,
	pub hds_ip_ext: u8,
	pub l4_hdr_type_etc: u8,
	pub vlan_info: __be16,
}

impl Default for mlx5_cqe64__bindgen_ty_1__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for mlx5_cqe64__bindgen_ty_1__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "mlx5_cqe64__bindgen_ty_1__bindgen_ty_1 {{ rsvd0: {:?}, rsvd4: {:?}, rsvd20: {:?} }}", self.rsvd0, self.rsvd4, self.rsvd20)
	}
}
