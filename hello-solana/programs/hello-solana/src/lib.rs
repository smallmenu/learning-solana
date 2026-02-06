use anchor_lang::prelude::*;

// Anchor 框架使用 Rust 宏来减少样板代码，并简化编写 Solana 程序所需的常见安全检查的实现。

// 指定程序的链上地址
declare_id!("84BJ6Bc1i8cmqH5xPqx1CSD4AAaaaWrwZPNyHhDJYUxi");

// 自定义错误类型，用于权限控制
#[error_code]
pub enum CustomError {
    #[msg("Unauthorized: Only the account owner can perform this operation")]
    Unauthorized,
}

// #[program] 指定包含程序指令逻辑的模块，该模块中的每个公共函数都对应一条可调用的指令。
#[program]
pub mod hello_solana {
    use super::*;

    // 每个处理程序的第一个参数是一个 Context<T> 类型，其中 T 是一个实现了 Accounts 相应 trait 的结构体，并指定指令所需的账户。
    // 包含一个名为 myinit 的指令，该指令创建一个新帐户 (NewAccount) 并用 u64 值对其进行初始化。
    pub fn myinit(ctx: Context<MyInit>, data: u64) -> Result<()> {
        msg!("Invoke myinit from: {:?}", ctx.program_id);

        msg!("New account: {}!", ctx.accounts.new_account.key());

        // 保存账户的创建者地址，用于后续权限控制
        ctx.accounts.new_account.owner = ctx.accounts.signer.key();

        ctx.accounts.new_account.data = data;

        msg!(
            "Account owner set to: {:?}, Changed data to: {}!",
            ctx.accounts.signer.key(),
            data
        );

        Ok(())
    }

    // mycalc 指令：对已创建的账户数据进行四则运算
    // 接收一个已初始化的账户和一个 u32 操作数
    // 执行四种计算：
    //   1. result1 = data + ops * 2
    //   2. result2 = data - ops
    //   3. result3 = data * ops
    //   4. result4 = data / ops
    // 将最终结果保存到账户的 data 字段中
    pub fn mycalc(ctx: Context<MyCalc>, ops: u32) -> Result<()> {
        msg!("Invoke mycalc from: {:?}", ctx.program_id);

        let current_data = ctx.accounts.my_account.data;

        msg!(
            "Current account data: {}, Operation value: {}",
            current_data,
            ops
        );

        // 将 u64 和 u32 转换为 u64 进行计算，避免溢出
        let ops_u64 = ops as u64;

        // 执行四则运算
        let result1 = current_data + ops_u64 * 2;
        let result2 = current_data.saturating_sub(ops_u64);
        let result3 = current_data.saturating_mul(ops_u64);
        let result4 = if ops_u64 != 0 {
            current_data / ops_u64
        } else {
            msg!("Warning: Division by zero, keeping original data");
            current_data
        };

        // 打印所有计算结果
        msg!("Calculation results:");
        msg!(
            "  data + ops * 2 = {} + {} * 2 = {}",
            current_data,
            ops,
            result1
        );
        msg!("  data - ops = {} - {} = {}", current_data, ops, result2);
        msg!("  data * ops = {} * {} = {}", current_data, ops, result3);
        msg!("  data / ops = {} / {} = {}", current_data, ops, result4);

        // 选择最后一个计算结果作为最终值保存到账户
        ctx.accounts.my_account.data = result4;

        Ok(())
    }
}

// #[derive(Accounts)]：Anchor框架的宏，为结构体实现 Accounts trait
// MyInit<'info> ：结构体名称，带有生命周期参数 'info ，这是 Anchor 处理账户时的标准做法
// MyInit 结构体实现了 Accounts 特性，其中结构体中的每个字段都代表初始化指令所需的帐户。
// 账户验证和约束，通过 #[account(..)] 属性，该属性位于实现了相应 Accounts特性的结构体字段
// Account、Signer、Program 是账户类型
// 当 Anchor 程序中的一条指令被调用时，程序会先验证所提供的账户，然后再执行该指令的逻辑。验证通过后，就可以使用该指令的 ctx.accounts 语法访问这些账户
#[derive(Accounts)]
pub struct MyInit<'info> {
    // #[account(...)] 账户属性宏，配置账户的创建和验证规则
    // 在 Anchor 程序中，账户鉴别器指的是每个账户类型唯一的 8 字节标识符
    // 现在 MyNewAccount 包含两个字段：
    //   - owner: Pubkey (32 字节)
    //   - data: u64 (8 字节)
    // 所以总空间 = 8 (鉴别器) + 32 (owner) + 8 (data) = 48 字节
    // init 指示需要初始化这个账户
    // payer = signer 指示 signer 账户支付初始化这个账户的费用
    #[account(init, payer = signer, space = 8 + 32 + 8)]
    pub new_account: Account<'info, MyNewAccount>,

    // 标记账户为可变的，因为它需要支付创建新账户的费用
    // 签名者账户类型，用于验证交易签名，确保交易是由合法用户发起的
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// MyCalc 结构体：定义 mycalc 指令所需的账户
// 验证调用者是账户的创建者（owner），只有创建者才能修改账户数据
#[derive(Accounts)]
pub struct MyCalc<'info> {
    // 签名者账户，用于验证调用者身份
    pub signer: Signer<'info>,

    // 传入一个已创建的账户，标记为可变以便修改其 data 字段
    // constraint 约束确保只有账户的创建者（owner）才能调用此指令
    #[account(
        mut,
        constraint = my_account.owner == signer.key() @ CustomError::Unauthorized
    )]
    pub my_account: Account<'info, MyNewAccount>,
}

// #[account]：应用于结构体，为程序创建自定义帐户类型
// 创建帐户时，帐户的程序所有者会自动设置为在 中指定的程序 declare_id。
#[account]
pub struct MyNewAccount {
    // 存储账户的创建者地址，用于权限控制，只有创建者才能修改这个账户的数据
    owner: Pubkey,
    data: u64,
}
