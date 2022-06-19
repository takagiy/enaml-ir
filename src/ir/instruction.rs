pub struct Instruction {
    pub opcode: Opcode,
}

impl Instruction {}

pub enum Opcode {
    Add,
    Mul,
    Sub,
    Div,
    Mod,
    AddMod,
    MulMod,
    Exp,
    Sext,

    Lt,
    Gt,
    Eq,
    IsZero,

    And,
    Or,
    Xor,
    Not,
    Byte,
    Shl,
    Shr,
    Sar,

    Sha3,

    Address,
    ExternalBalance,
    TxOrigin,
    Caller,

    CallValue,
    CallData,
    CallDataSize,
    CopyCallData,

    CodeSize,
    CopyCode,

    TxGasPrice,

    ExternalCodeSize,
    CopyExternalCode,

    ReturnDataSize,
    CopyReturnData,

    ExternalCodeHash,
    BlockHash,

    BlockCoinbase,
    BlockTimestamp,
    BlockNumber,
    BlockDifficulty,
    BlockGasLimit,
    ChainId,
    Balance,
    BaseFee,

    Load,
    Store,
    Store8,

    LoadStorage,
    StoreStorage,

    Jump,

    MemorySize,

    GasLeft,

    Create,

    Call,
    CallCode,
    Finish,
    CallDelegate,
    CallStatic,
    Revert,
    SelfDestruct,
}
