require("dotenv").config();
import * as solanaWeb3 from "@solana/web3.js";
import BigNumber from "bignumber.js";
import { createPDAAccount } from "./lock";
import { establishConnection, getBalance, getKeyPair, integerToByteArray, lamports, sleep } from "./utils";

const programId = process.env.PROGRAM_ID;
const main = async (): Promise<unknown> => {
    try {
        const connection = await establishConnection();
        const secretKey = Uint8Array.from(process.env.SECRET_KEY?.split(",").map(x => Number(x)) as number[]);
        const locker = getKeyPair(secretKey);

        let balance = await getBalance(connection, locker);
        console.log(`Balance of Locker: ${locker.publicKey}: ${balance}`);

        const lockerPDA = await createPDAAccount(connection, new solanaWeb3.PublicKey(programId as string), locker, lamports(1));

        const data = Uint8Array.from([
            0,
            ...integerToByteArray(lamports(1))
        ])

        const lockInstruction = new solanaWeb3.TransactionInstruction({
            keys: [
                { pubkey: locker.publicKey, isSigner: true, isWritable: false },
                { pubkey: lockerPDA.keypair.publicKey, isSigner: false, isWritable: true }
            ],
            programId: new solanaWeb3.PublicKey(programId as string),
            data: Buffer.from(data)
        })

        await solanaWeb3.sendAndConfirmTransaction(connection, new solanaWeb3.Transaction().add(lockInstruction), [locker]);

        const accountInfo = await connection.getAccountInfo(lockerPDA.keypair.publicKey);
        let storedData = accountInfo?.data.toString("hex") as string;
        const isInitialized = storedData.slice(0, 2);
        const publicKey = storedData.slice(2, 2 + 64);
        const balanceLocked = await getBalance(connection, lockerPDA.keypair);
        const time = storedData.slice(84);

        balance = await getBalance(connection, locker);
        console.log(`Balance of Locker: ${locker.publicKey}: ${balance}`);
        console.log('Raw: ', storedData);
        console.log('Init: ', new BigNumber(isInitialized).toString());
        console.log('On Chain Public Key: ', publicKey, Buffer.from(lockerPDA.keypair.publicKey.toBytes()).toString("hex"))
        console.log('Amount: ', balanceLocked);
        console.log('Time: ', time);

        await sleep(1000 * 60 * 60)
        const unlockInstruction = new solanaWeb3.TransactionInstruction({
            keys: [
                { pubkey: locker.publicKey, isSigner: true, isWritable: false },
                { pubkey: lockerPDA.keypair.publicKey, isSigner: false, isWritable: true }
            ],
            programId: new solanaWeb3.PublicKey(programId as string),
            data: Buffer.from(Uint8Array.from([1]))
        });

        const inx_res = await solanaWeb3.sendAndConfirmTransaction(connection, new solanaWeb3.Transaction().add(unlockInstruction), [locker]);
        console.log(inx_res)

        balance = await getBalance(connection, locker);
        console.log(`Balance of Locker: ${locker.publicKey}: ${balance}`);

        return Promise.resolve();
    } catch (err) {
        console.error('[Main] ', err);
    }
}

main();