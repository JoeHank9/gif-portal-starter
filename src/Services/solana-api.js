import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { Program, AnchorProvider, web3 } from '@project-serum/anchor';
import { Buffer } from "buffer";
import idl from '../idl.json';
import kp from '../keypair.json'

// SystemProgram is a reference to the Solana runtime!
const { SystemProgram } = web3;
// // Create a keypair for the account that will hold the GIF data.
const arr = Object.values(kp._keypair.secretKey)
const secret = new Uint8Array(arr)
const baseAccount = web3.Keypair.fromSecretKey(secret)
// // Get our program's id from the IDL file.
const programID = new PublicKey(idl.metadata.address);
// // Set our network to devnet.
const network = clusterApiUrl('devnet');
// // Controls how we want to acknowledge when a transaction is "done".
const opts = {
    preflightCommitment: "processed"
}

export const checkIfWalletIsConnected = async () => {
    try {
        const { solana } = window;

        if (solana) {
            if (solana.isPhantom) {
                console.log('Phantom wallet found!');
                const response = await solana.connect({ onlyIfTrusted: true });
                console.log(
                    'Connected with Public Key:',
                    response.publicKey.toString()
                );
                /*
                 * Set the user's publicKey in state to be used later!
                 */
                return response.publicKey.toString();
            }
        } else {
            alert('Solana object not found! Get a Phantom Wallet 👻');
        }
    } catch (error) {
        console.error(error);
    }
};

/*
* Let's connect the wallet
*/
export const connectWallet = async () => {
    const { solana } = window;

    if (solana) {
        const response = await solana.connect();
        console.log('Connected with Public Key:', response.publicKey.toString());
        return response.publicKey.toString();
    }
};

export const getProvider = () => {
    const connection = new Connection(network, opts.preflightCommitment);
    const provider = new AnchorProvider(
      connection, window.solana, opts.preflightCommitment,
    );
    return provider;
};
