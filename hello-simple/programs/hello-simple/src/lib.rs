use anchor_lang::prelude::*;

declare_id!("9PEM6jn8AbpUBycNL1dzWnmEewYW6QAXuWANVB1eyd9r");

#[program]
pub mod hello_simple {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        
        // 声明变量
        let a: i32 = 10;
        let b: i32 = 3;
        
        // 执行四则运算
        let add_result = a + b;
        let sub_result = a - b;
        let mul_result = a * b;
        let div_result = a / b;
        
        // 输出日志
        msg!("Var a: {}, Var b: {}", a, b);
        msg!("Add: {} + {} = {}", a, b, add_result);
        msg!("Sub: {} - {} = {}", a, b, sub_result);
        msg!("Mul: {} * {} = {}", a, b, mul_result);
        msg!("Div: {} / {} = {}", a, b, div_result);
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
