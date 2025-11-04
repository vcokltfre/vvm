pub const OP_EXIT: u8 = 0x00;
pub const OP_EXIT_IMMEDIATE: u8 = 0x01;

pub const OP_PUSH_INT: u8 = 0x10;
pub const OP_PUSH_UINT: u8 = 0x11;
pub const OP_PUSH_FLOAT: u8 = 0x12;
pub const OP_PUSH_BOOL: u8 = 0x13;
pub const OP_PUSH_STRING: u8 = 0x14;
pub const OP_POP: u8 = 0x15;
pub const OP_DUP: u8 = 0x16;
pub const OP_SWAP: u8 = 0x17;

pub const OP_ADD: u8 = 0x20;
pub const OP_ADD_I: u8 = 0x21;
pub const OP_ADD_U: u8 = 0x22;
pub const OP_ADD_F: u8 = 0x23;
pub const OP_SUB: u8 = 0x24;
pub const OP_SUB_I: u8 = 0x25;
pub const OP_SUB_U: u8 = 0x26;
pub const OP_SUB_F: u8 = 0x27;
pub const OP_MUL: u8 = 0x28;
pub const OP_MUL_I: u8 = 0x29;
pub const OP_MUL_U: u8 = 0x2a;
pub const OP_MUL_F: u8 = 0x2b;
pub const OP_DIV: u8 = 0x2c;
pub const OP_DIV_I: u8 = 0x2d;
pub const OP_DIV_U: u8 = 0x2e;
pub const OP_DIV_F: u8 = 0x2f;
pub const OP_MOD: u8 = 0x30;
pub const OP_MOD_I: u8 = 0x31;
pub const OP_MOD_U: u8 = 0x32;
pub const OP_EXP: u8 = 0x33;
pub const OP_EXP_I: u8 = 0x34;
pub const OP_EXP_U: u8 = 0x35;
pub const OP_EXP_F: u8 = 0x36;

pub const OP_LOAD: u8 = 0x40;
pub const OP_LOAD_IMM: u8 = 0x41;
pub const OP_STORE: u8 = 0x42;
pub const OP_STORE_IMM: u8 = 0x43;
pub const OP_FREE: u8 = 0x44;
pub const OP_FREE_IMM: u8 = 0x45;

pub const OP_CMP_EQUAL: u8 = 0x50;
pub const OP_CMP_NOT_EQUAL: u8 = 0x51;
pub const OP_CMP_GREATER_THAN: u8 = 0x52;
pub const OP_CMP_LESS_THAN: u8 = 0x53;
pub const OP_CMP_GREATER_EQUAL: u8 = 0x54;
pub const OP_CMP_LESS_EQUAL: u8 = 0x55;

pub const OP_JUMP: u8 = 0x60;
pub const OP_JUMP_IF: u8 = 0x61;
pub const OP_CALL: u8 = 0x62;
pub const OP_CALL_NATIVE: u8 = 0x63;
pub const OP_RET: u8 = 0x64;

pub const OP_LABEL: u8 = 0x70;
