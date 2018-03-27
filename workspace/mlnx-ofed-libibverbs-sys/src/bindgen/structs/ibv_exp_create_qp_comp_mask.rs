// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ibv_exp_create_qp_comp_mask(pub u32);

impl BitOr<ibv_exp_create_qp_comp_mask> for ibv_exp_create_qp_comp_mask
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		ibv_exp_create_qp_comp_mask(self.0 | other.0)
	}
}

impl BitOrAssign for ibv_exp_create_qp_comp_mask
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: ibv_exp_create_qp_comp_mask)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<ibv_exp_create_qp_comp_mask> for ibv_exp_create_qp_comp_mask
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		ibv_exp_create_qp_comp_mask(self.0 & other.0)
	}
}

impl BitAndAssign for ibv_exp_create_qp_comp_mask
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: ibv_exp_create_qp_comp_mask)
	{
		self.0 &= rhs.0;
	}
}
