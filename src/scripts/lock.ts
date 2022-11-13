import * as solanaWeb3 from "@solana/web3.js";

export const createPDAAccount = async (
  connection: solanaWeb3.Connection,
  programId: solanaWeb3.PublicKey,
  author: solanaWeb3.Keypair,
  lamports: number
): Promise<{ signature: string; keypair: solanaWeb3.Keypair }> => {
  const keypair = solanaWeb3.Keypair.generate();
  const transaction = new solanaWeb3.Transaction();
  const instruction = solanaWeb3.SystemProgram.createAccount({
    fromPubkey: author.publicKey,
    newAccountPubkey: keypair.publicKey,
    space: 49,
    lamports,
    programId,
  });
  transaction.add(instruction);
  var signature = await solanaWeb3.sendAndConfirmTransaction(
    connection,
    transaction,
    [author, keypair]
  );
  return { signature, keypair };
};
