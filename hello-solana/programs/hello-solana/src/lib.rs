use anchor_lang::prelude::*;

// Anchor 框架使用 Rust 宏来减少样板代码，并简化编写 Solana 程序所需的常见安全检查的实现。

// 指定程序的链上地址
declare_id!("84BJ6Bc1i8cmqH5xPqx1CSD4AAaaaWrwZPNyHhDJYUxi");

// #[program] 指定包含程序指令逻辑的模块，该模块中的每个公共函数都对应一条可调用的指令。
#[program]
pub mod hello_solana {
    use super::*;

    // 每个处理程序的第一个参数是一个 Context<T> 类型，其中 T 是一个实现了 Accounts 相应 trait 的结构体，并指定指令所需的账户。
    // 包含一个名为 myinit 的指令，该指令创建一个新帐户 (NewAccount) 并用 u64 值对其进行初始化。
    pub fn myinit(ctx: Context<MyInit>, data: u64) -> Result<()> {
        msg!("Invoke from: {:?}", ctx.program_id);

        msg!("New account: {}!", ctx.accounts.new_account.key());

        ctx.accounts.new_account.data = data;

        msg!("Changed data to: {}!", data);

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
    // 在 Anchor 程序中创建帐户时，必须为鉴别器分配 8 个字节。所以一共是 16 个字节，因为要存储 u64
    // init 指示需要初始化这个账户
    // payer = signer 指示 signer 账户支付初始化这个账户的费用
    // space = 8 + 8 指示这个账户需要分配 8 个字节的空间来存储 NewAccount 结构体
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, MyNewAccount>,

    // 标记账户为可变的，因为它需要支付创建新账户的费用
    // 签名者账户类型，用于验证交易签名，确保交易是由合法用户发起的
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// #[account]：应用于结构体，为程序创建自定义帐户类型
// 创建帐户时，帐户的程序所有者会自动设置为在 中指定的程序 declare_id。
#[account]
pub struct MyNewAccount {
    data: u64,
}
