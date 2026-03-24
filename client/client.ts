const { SystemProgram, PublicKey } = web3;

const description = "Build on Solana v2";

const [intentPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("intent"),
    pg.wallet.publicKey.toBuffer(),
    Buffer.from(description),
  ],
  pg.PROGRAM_ID
);

// Paso 1: Registrar intención
const tx1 = await pg.program.methods
  .registerIntent(description)
  .accounts({
    intent: intentPda,
    owner: pg.wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();

console.log("Intent registrado!");
console.log("TX1:", tx1);
console.log("https://explorer.solana.com/tx/" + tx1 + "?cluster=devnet");

// Paso 2: Cumplir intención
const tx2 = await pg.program.methods
  .fulfillIntent()
  .accounts({
    intent: intentPda,
    owner: pg.wallet.publicKey,
  })
  .rpc();

console.log("Intent cumplido!");
console.log("TX2:", tx2);
console.log("https://explorer.solana.com/tx/" + tx2 + "?cluster=devnet");
