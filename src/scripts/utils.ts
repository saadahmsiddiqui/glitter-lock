import * as solanaWeb3 from "@solana/web3.js";
import BigNumber from "bignumber.js";

export const establishConnection = async (): Promise<solanaWeb3.Connection> => {
  const rpcUrl = "https://api.devnet.solana.com";
  const connection = new solanaWeb3.Connection(rpcUrl, "confirmed");
  console.log("Connection to cluster established:", rpcUrl);
  return connection;
};

export const getKeyPair = (secretKey: Uint8Array): solanaWeb3.Keypair => {
  const keyPair = solanaWeb3.Keypair.fromSecretKey(secretKey);
  return keyPair;
};

export const getBalance = async (
  connection: solanaWeb3.Connection,
  keypair: solanaWeb3.Keypair
): Promise<number> => {
  const balance = await connection.getBalance(keypair.publicKey);
  return balance;
};

export const integerToByteArray = (integer: any) => {
  var byteArray = [0, 0, 0, 0, 0, 0, 0, 0];

  for (var index = 0; index < byteArray.length; index++) {
    var byte = integer & 0xff;
    byteArray[index] = byte;
    integer = (integer - byte) / 256;
  }

  return byteArray;
};

export const lamports = (amount: number) =>
  new BigNumber(amount).times(new BigNumber(10).pow(9)).toNumber();

export const sleep = (ms: number): Promise<unknown> => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};
