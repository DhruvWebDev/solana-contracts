import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Multisig } from "../target/types/multisig";
import {
  SystemProgram,
  Keypair,
  PublicKey,
  TransactionInstruction,
} from "@solana/web3.js";
import type { Multisig } from "../target/types/multisig";
// utils ----------------------
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const wallet = provider.wallet as anchor.Wallet;
const connection = provider.connection;
const program = anchor.workspace.Multisig as Program<Multisig>;
const pid = program.programId;
//multisig signer and pda ----------------------
const multisigSigner = web3.Keypair.generate();
const [multisigPDA, bump] = anchor.web3.PublicKey.findProgramAddressSync(
  [multisigSigner.publicKey.toBuffer()],
  program.programId
);
let multisigSize = 250;

//tx -----------
const txAccount = web3.Keypair.generate();
const txsize = 1000;

//Mock Accounts --------------
let ownerA = web3.Keypair.generate();
let ownerB = web3.Keypair.generate();
let ownerC = web3.Keypair.generate();
let ownerD = web3.Keypair.generate();
describe("test for my multisig contract", async () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Multisig as anchor.Program<Multisig>;
  
  it("init multisig", async () => {
    let ownersArr = [
      ownerA.publicKey,
      ownerB.publicKey,
      ownerC.publicKey,
      ownerD.publicKey,
    ];
    const threshold = new anchor.BN(1);
    const result = await program.methods
      .initializeMultisig(ownersArr, threshold, bump)
      .accounts({ multisig: multisigSigner.publicKey })
      .preInstructions([
        await program.account.multisig.createInstruction(
          multisigSigner,
          multisigSize
        ),
      ])
      .signers([multisigSigner])
      .rpc();
    console.log("here", result);
  });

  it("create tx", async () => {
    const new_owners = [ownerA, ownerB];
    const accounts = [
      {
        pubkey: multisigSigner.publicKey,
        isWritable: true,
        isSigner: false,
      },
      {
        pubkey: multisigPDA,
        isWritable: false,
        isSigner: true,
      },
    ];
    const data = program.coder.instruction.encode("change_owners", {
      new_owners,
    });
    console.log(txAccount);
    await program.methods
      .createTx(pid, data, accounts)
      .accounts({
        multisig: multisigSigner.publicKey,
        transaction: txAccount.publicKey,
        proposer: ownerA.publicKey,
      })
      .preInstructions([
        await program.account.transaction.createInstruction(txAccount, txsize),
      ])
      .signers([ownerA, txAccount])
      .rpc();
  });

  it("approve", () => {

  });

  it("exec tx", () => {
    
  });
});
