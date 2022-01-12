const {
  Connection,
  sendAndConfirmTransaction,
  Keypair,
  Transaction,
  SystemProgram,
  PublicKey,
  TransactionInstruction,
} = require("@solana/web3.js");

const BN = require("bn.js");

const main = async () => {
  var args = process.argv.slice(2);
  const programId = new PublicKey(args[0]);
  const echo = args[1];

  const connection = new Connection("http://127.0.0.1:8899");
  // const connection = new Connection("https://api.devnet.solana.com");

  const feePayer = new Keypair();
  const echoBuffer = new Keypair();

  console.log("Requesting Airdrop of 1 SOL...");
  await connection.requestAirdrop(feePayer.publicKey, 2e9);
  console.log("Airdrop received");

  let createIx = SystemProgram.createAccount({
    fromPubkey: feePayer.publicKey,
    newAccountPubkey: echoBuffer.publicKey,
    /** Amount of lamports to transfer to the created account */
    lamports: await connection.getMinimumBalanceForRentExemption(echo.length),
    /** Amount of space in bytes to allocate to the created account */
    space: echo.length,
    /** Public key of the program to assign as the owner of the created account */
    programId: programId,
  });

  // 0th instruction
  const idx = Buffer.from(new Uint8Array([0]));
  const messageLen = Buffer.from(
    new Uint8Array(new BN(echo.length).toArray("le", 4))
  );
  const message = Buffer.from(echo, "ascii");

  let echoIx = new TransactionInstruction({
    keys: [
      {
        pubkey: echoBuffer.publicKey,
        isSigner: false,
        isWritable: true,
      },
    ],
    programId: programId,
    data: Buffer.concat([
      Buffer.from(new Uint8Array([0])),
      messageLen,
      message,
    ]),
  });

  // 1st instruction

  let authority = feePayer;
  // const authority = new Keypair();

  const buffer_seed = 10;
  const buffer_seed_buf = Buffer.from(
    new Uint8Array(new BN(buffer_seed).toArray("le", 8))
  );

  const authorized_buffer = (
    await PublicKey.findProgramAddress(
      [
        Buffer.from("authority"),
        authority.publicKey.toBuffer(),
        buffer_seed_buf,
      ],
      programId
    )
  )[0];
  const b = Buffer.from("authority", "UTF-8");
  console.log(b, b.toString());
  console.log("tx1");
  let echoIx1 = new TransactionInstruction({
    keys: [
      {
        pubkey: authorized_buffer,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: authority.publicKey,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId: programId,
    data: Buffer.concat([
      Buffer.from(new Uint8Array([1])),
      buffer_seed_buf,
      Buffer.from(new Uint8Array(new BN(echo.length + 9).toArray("le", 8))),
    ]),
  });
  console.log("tx2");
  let echoIx2 = new TransactionInstruction({
    keys: [
      {
        pubkey: authorized_buffer,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: authority.publicKey,
        isSigner: true,
        isWritable: false,
      },
    ],
    programId: programId,
    data: Buffer.concat([
      Buffer.from(new Uint8Array([2])),
      messageLen,
      message,
    ]),
  });
  console.log("autority key frontend ", authority.publicKey);
  // end of instructions

  let tx = new Transaction();
  tx.add(echoIx1).add(echoIx2);
  // tx.add(createIx).add(echoIx);
  let txid = await sendAndConfirmTransaction(connection, tx, [authority], {
    // let txid = await sendAndConfirmTransaction(connection, tx, [feePayer, echoBuffer], {
    skipPreflight: true,
    preflightCommitment: "confirmed",
    confirmation: "confirmed",
    commitment: "confirmed",
  });

  console.log(`https://explorer.solana.com/tx/${txid}?cluster=devnet`);

  // data = (await connection.getAccountInfo(echoBuffer.publicKey, "confirmed"))
  data = (await connection.getAccountInfo(authorized_buffer, "confirmed")).data;
  console.log("Echo Buffer Text:", data.toString());
};

main()
  .then(() => {
    console.log("Success");
  })
  .catch((e) => {
    console.error(e);
  });
