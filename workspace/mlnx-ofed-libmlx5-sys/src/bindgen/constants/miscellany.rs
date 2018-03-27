// This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


pub const MLX5E_CQE_FORMAT_MASK: u32 = 12;
pub const MLX5_CALC_FLOAT64_ADD: _bindgen_ty_11 = 2;
pub const MLX5_CALC_UINT64_ADD: _bindgen_ty_11 = 1;
pub const MLX5_CALC_UINT64_AND: _bindgen_ty_11 = 4;
pub const MLX5_CALC_UINT64_MAXLOC: _bindgen_ty_11 = 3;
pub const MLX5_CALC_UINT64_OR: _bindgen_ty_11 = 5;
pub const MLX5_CALC_UINT64_XOR: _bindgen_ty_11 = 6;
pub const MLX5_COMPRESSED: _bindgen_ty_11 = 3;
pub const MLX5_CQE_INVALID: _bindgen_ty_11 = 15;
pub const MLX5_CQE_L2_OK: _bindgen_ty_11 = 1;
pub const MLX5_CQE_L3_HDR_TYPE_IPV4: _bindgen_ty_11 = 8;
pub const MLX5_CQE_L3_HDR_TYPE_IPV6: _bindgen_ty_11 = 4;
pub const MLX5_CQE_L3_HDR_TYPE_MASK: _bindgen_ty_11 = 12;
pub const MLX5_CQE_L3_HDR_TYPE_NONE: _bindgen_ty_11 = 0;
pub const MLX5_CQE_L3_OK: _bindgen_ty_11 = 2;
pub const MLX5_CQE_L4_HDR_TYPE_MASK: _bindgen_ty_11 = 112;
pub const MLX5_CQE_L4_HDR_TYPE_TCP: _bindgen_ty_11 = 16;
pub const MLX5_CQE_L4_HDR_TYPE_TCP_ACK: _bindgen_ty_11 = 64;
pub const MLX5_CQE_L4_HDR_TYPE_TCP_EMP_ACK: _bindgen_ty_11 = 48;
pub const MLX5_CQE_L4_HDR_TYPE_UDP: _bindgen_ty_11 = 32;
pub const MLX5_CQE_L4_OK: _bindgen_ty_11 = 4;
pub const MLX5_CQE_OPCODE_ERROR: _bindgen_ty_11 = 30;
pub const MLX5_CQE_OPCODE_RESIZE: _bindgen_ty_11 = 22;
pub const MLX5_CQE_OWNER_MASK: _bindgen_ty_11 = 1;
pub const MLX5_CQE_REQ: _bindgen_ty_11 = 0;
pub const MLX5_CQE_REQ_ERR: _bindgen_ty_11 = 13;
pub const MLX5_CQE_RESIZE_CQ: _bindgen_ty_11 = 5;
pub const MLX5_CQE_RESP_ERR: _bindgen_ty_11 = 14;
pub const MLX5_CQE_RESP_SEND: _bindgen_ty_11 = 2;
pub const MLX5_CQE_RESP_SEND_IMM: _bindgen_ty_11 = 3;
pub const MLX5_CQE_RESP_SEND_INV: _bindgen_ty_11 = 4;
pub const MLX5_CQE_RESP_WR_IMM: _bindgen_ty_11 = 1;
pub const MLX5_CQE_SIG_ERR: _bindgen_ty_11 = 12;
pub const MLX5_CQE_SYNDROME_BAD_RESP_ERR: _bindgen_ty_11 = 16;
pub const MLX5_CQE_SYNDROME_LOCAL_ACCESS_ERR: _bindgen_ty_11 = 17;
pub const MLX5_CQE_SYNDROME_LOCAL_LENGTH_ERR: _bindgen_ty_11 = 1;
pub const MLX5_CQE_SYNDROME_LOCAL_PROT_ERR: _bindgen_ty_11 = 4;
pub const MLX5_CQE_SYNDROME_LOCAL_QP_OP_ERR: _bindgen_ty_11 = 2;
pub const MLX5_CQE_SYNDROME_MW_BIND_ERR: _bindgen_ty_11 = 6;
pub const MLX5_CQE_SYNDROME_REMOTE_ABORTED_ERR: _bindgen_ty_11 = 34;
pub const MLX5_CQE_SYNDROME_REMOTE_ACCESS_ERR: _bindgen_ty_11 = 19;
pub const MLX5_CQE_SYNDROME_REMOTE_INVAL_REQ_ERR: _bindgen_ty_11 = 18;
pub const MLX5_CQE_SYNDROME_REMOTE_OP_ERR: _bindgen_ty_11 = 20;
pub const MLX5_CQE_SYNDROME_RNR_RETRY_EXC_ERR: _bindgen_ty_11 = 22;
pub const MLX5_CQE_SYNDROME_TRANSPORT_RETRY_EXC_ERR: _bindgen_ty_11 = 21;
pub const MLX5_CQE_SYNDROME_WR_FLUSH_ERR: _bindgen_ty_11 = 5;
pub const MLX5_CQ_DB_REQ_NOT: u32 = 0;
pub const MLX5_CQ_DB_REQ_NOT_SOL: u32 = 16777216;
pub const MLX5_CQ_DOORBELL: _bindgen_ty_11 = 32;
pub const MLX5_ETH_INLINE_HEADER_SIZE: _bindgen_ty_11 = 18;
pub const MLX5_ETH_VLAN_INLINE_HEADER_SIZE: _bindgen_ty_11 = 18;
pub const MLX5_ETH_WQE_L3_CSUM: _bindgen_ty_11 = 64;
pub const MLX5_ETH_WQE_L4_CSUM: _bindgen_ty_11 = 128;
pub const MLX5_EXTENDED_UD_AV: _bindgen_ty_11 = 2147483648;
pub const MLX5_INLINE_DATA32_SEG: _bindgen_ty_11 = 1;
pub const MLX5_INLINE_DATA64_SEG: _bindgen_ty_11 = 2;
pub const MLX5_INLINE_SCATTER_32: _bindgen_ty_11 = 4;
pub const MLX5_INLINE_SCATTER_64: _bindgen_ty_11 = 8;
pub const MLX5_INLINE_SEG: _bindgen_ty_11 = 2147483648;
pub const MLX5_INVALID_LKEY: _bindgen_ty_11 = 256;
pub const MLX5_MINI_ARR_SIZE: _bindgen_ty_11 = 8;
pub const MLX5_NO_INLINE_DATA: _bindgen_ty_11 = 0;
pub const MLX5_OPCODE_ATOMIC_CS: _bindgen_ty_11 = 17;
pub const MLX5_OPCODE_ATOMIC_FA: _bindgen_ty_11 = 18;
pub const MLX5_OPCODE_ATOMIC_MASKED_CS: _bindgen_ty_11 = 20;
pub const MLX5_OPCODE_ATOMIC_MASKED_FA: _bindgen_ty_11 = 21;
pub const MLX5_OPCODE_CONFIG_CMD: _bindgen_ty_11 = 31;
pub const MLX5_OPCODE_CQE_WAIT: _bindgen_ty_11 = 15;
pub const MLX5_OPCODE_FMR: _bindgen_ty_11 = 25;
pub const MLX5_OPCODE_LOCAL_INVAL: _bindgen_ty_11 = 27;
pub const MLX5_OPCODE_NOP: _bindgen_ty_11 = 0;
pub const MLX5_OPCODE_RDMA_READ: _bindgen_ty_11 = 16;
pub const MLX5_OPCODE_RDMA_WRITE: _bindgen_ty_11 = 8;
pub const MLX5_OPCODE_RDMA_WRITE_IMM: _bindgen_ty_11 = 9;
pub const MLX5_OPCODE_RECV_ENABLE: _bindgen_ty_11 = 22;
pub const MLX5_OPCODE_SEND: _bindgen_ty_11 = 10;
pub const MLX5_OPCODE_SEND_ENABLE: _bindgen_ty_11 = 23;
pub const MLX5_OPCODE_SEND_IMM: _bindgen_ty_11 = 11;
pub const MLX5_OPCODE_SEND_INVAL: _bindgen_ty_11 = 1;
pub const MLX5_OPCODE_TSO: _bindgen_ty_11 = 14;
pub const MLX5_OPC_MOD_MPW: _bindgen_ty_11 = 1;
pub const MLX5_QP_PEER_VA_ID_MAX: _bindgen_ty_11 = 2;
pub const MLX5_RCV_DBR: _bindgen_ty_11 = 0;
pub const MLX5_RECV_OPCODE_RDMA_WRITE_IMM: _bindgen_ty_11 = 0;
pub const MLX5_RECV_OPCODE_SEND: _bindgen_ty_11 = 1;
pub const MLX5_RECV_OPCODE_SEND_IMM: _bindgen_ty_11 = 2;
pub const MLX5_RECV_OPCODE_SEND_INVAL: _bindgen_ty_11 = 3;
pub const MLX5_SEND_WQE_BB: _bindgen_ty_11 = 64;
pub const MLX5_SEND_WQE_SHIFT: _bindgen_ty_11 = 6;
pub const MLX5_SND_DBR: _bindgen_ty_11 = 1;
pub const MLX5_SRQ_FLAG_SIGNATURE: _bindgen_ty_11 = 1;
pub const MLX5_WQE_CTRL_CQ_UPDATE: _bindgen_ty_11 = 8;
pub const MLX5_WQE_CTRL_FENCE: _bindgen_ty_11 = 128;
pub const MLX5_WQE_CTRL_SOLICITED: _bindgen_ty_11 = 2;
pub const mlx5_alloc_type_MXM_MLX5_ALLOC_TYPE_DUMMY: mlx5_alloc_type = 0;
pub const mlx5_db_method_MXM_MLX5_DB_TYPE_DUMMY: mlx5_db_method = 0;
pub const mlx5_lock_state_MXM_MLX5_LOCK_STATE_TYPE_DUMMY: mlx5_lock_state = 0;
pub const mlx5_lock_type_MXM_MLX5_LOCK_TYPE_DUMMY: mlx5_lock_type = 0;
pub const mlx5_rsc_type_MXM_MLX5_RSC_TYPE_DUMMY: mlx5_rsc_type = 0;