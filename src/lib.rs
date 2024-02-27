use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

// 定义计数器数据结构
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    pub value: u32,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // 获取账户迭代器
    let accounts_iter = &mut accounts.iter();

    // 获取计数器账户（第一个账户）
    let counter_account = next_account_info(accounts_iter)?;

    // 确保账户可写且由当前程序拥有
    if !counter_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }
    if counter_account.owner != _program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // 反序列化数据
    let mut counter = Counter::try_from_slice(&counter_account.data.borrow())?;

    // 执行 +1 操作
    counter.value += 1;

    // 序列化并保存数据
    counter.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;

    Ok(())
}