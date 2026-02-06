import * as anchor from "@coral-xyz/anchor";
import {BN, getProvider, Program, web3} from "@coral-xyz/anchor";
import {HelloSolana} from "../target/types/hello_solana";

describe("hello-solana", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.helloSolana as Program<HelloSolana>;

    it("myinit ok", async () => {

        const newAccountKp = new web3.Keypair();

        const data = new BN(42);

        // Add your test here.
        const tx = await program.methods.myinit(data)
            .accounts({
                newAccount: newAccountKp.publicKey,
                // 签名者账户，用于支付创建新账户的费用，智能合约会验证该账户是否对交易进行了签名
                signer: getProvider().wallet.publicKey,
                // systemProgram: web3.SystemProgram.programId, // 可以不用写
            })
            // 注意：这里没有包含钱包的密钥对，因为 Anchor 会自动使用 getProvider().wallet 作为默认签名者
            .signers([newAccountKp])
            .rpc()

        console.log("Your transaction signature", tx);
    });

    it("mycalc init ok", async () => {
        // 首先创建一个账户，初始化数据为 100
        const newAccountKp = new web3.Keypair();
        const initialData = new BN(42);

        // 步骤1：初始化账户
        const initTx = await program.methods.myinit(initialData)
            .accounts({
                newAccount: newAccountKp.publicKey,
                signer: getProvider().wallet.publicKey,
            })
            .signers([newAccountKp])
            .rpc();

        console.log("Init transaction signature:", initTx);

        // 步骤2：调用 mycalc 指令，传入操作数 ops = 5
        const ops = new BN(5);

        const calcTx = await program.methods.mycalc(ops)
            .accounts({
                myAccount: newAccountKp.publicKey,
                // 必须传入 signer，验证调用者是账户的创建者
                signer: getProvider().wallet.publicKey,
            })
            .rpc();

        console.log("Mycalc transaction signature:", calcTx);

        // 步骤3：验证结果
        // 获取账户的最新数据
        const account = await program.account.myNewAccount.fetch(newAccountKp.publicKey);
        console.log("Account data after mycalc:", account.data.toString());

        // 预期计算结果：
        // data + ops * 2 = 42 + 5 * 2 = 52
        // data - ops = 42 - 5 = 37
        // data * ops = 42 * 5 = 210
        // data / ops = 42 / 5 = 8
        // 保存的是最后的结果 (data / ops) = 20
        console.log("Expected final result (data / ops): 20");
        console.log("Actual result:", account.data.toNumber());
    });

    it("mycalc with pubkey ok", async () => {
        // 使用固定的 pubkey 直接调用 mycalc
        // 现在使用固定的 pubkey 调用 mycalc（这里使用刚创建的账户的 pubkey）
        const fixedAccountPubkey = new web3.PublicKey("2D7y1ujHacMzNhDJfEmwc42FRcNSZk9wJHchojrH2kXE")
        const ops = new BN(10);

        const account1 = await program.account.myNewAccount.fetch(fixedAccountPubkey);
        console.log("原始数据", account1.data);
        console.log("操作数:", account1.data.toNumber());

        const calcTx = await program.methods.mycalc(ops)
            .accounts({
                myAccount: fixedAccountPubkey,
                signer: getProvider().wallet.publicKey,
            })
            .rpc();

        console.log("✓ Mycalc transaction signature:", calcTx);

        // 验证结果
        const account2 = await program.account.myNewAccount.fetch(fixedAccountPubkey);
        console.log("\n✓ 计算结果:");
        console.log("实际最终结果:", account2.data.toNumber());
    });
});
