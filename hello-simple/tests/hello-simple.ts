import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {HelloSimple} from "../target/types/hello_simple";

describe("hello-simple", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.helloSimple as Program<HelloSimple>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
