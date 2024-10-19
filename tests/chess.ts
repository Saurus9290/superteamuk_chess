import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { chess } from "../target/types/chess";

describe("chess", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.chess as Program<chess>;
  const user = PublicKey.findProgramAddressSync(
    [Buffer.from("user"), program.provider.publicKey.toBuffer()],
    program.programId
  )[0];

  it("Initialize User", async () => {
    // Add your test here.
    const tx = await program.methods
      .initializeUser()
      .accounts({
        user,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });
});
