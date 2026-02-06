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
});
