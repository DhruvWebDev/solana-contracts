import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { FavouriteCar } from "../target/types/favourite_car";
describe("Test", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.FavouriteCar as anchor.Program<FavouriteCar>;
  
  it("initialize", async () => {
    // Generate keypair for the new account
    const newAccountKp = new web3.Keypair();

    // Correct PDA derivation (Matching Rust code)
    const [favoritesPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), newAccountKp.publicKey.toBuffer()],
      new web3.PublicKey("DXWFenFFGXHgve9HeDNFbDArMrdqNHa4UBKCsqV7DybS")
    );

    // Transaction call
    const txHash = await program.methods
      .setFavCar("Alto") // Fix: function name matches Rust
      .accounts({
        user: newAccountKp.publicKey,
        favouriteCarPda: favoritesPda, // Fix: Lowercase "d"
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([newAccountKp])
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Fetch stored data
    const dataFromPda = await program.account.favouriteCarStruct.fetch(
      favoritesPda
    );
    console.log(dataFromPda);
  });
});
