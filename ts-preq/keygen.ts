import { Keypair } from "@solana/web3.js";
import bs58 from 'bs58'

import promptSync from "prompt-sync";
const prompt = promptSync(); 
let kp = Keypair.generate()

console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`)


console.log(`[${kp.secretKey}]`)






const wallet = Uint8Array.from([
  34, 46, 55, 124, 141, 190, 24, 204, 134, 91, 70, 184, 161, 181, 44, 122,
  15, 172, 63, 62, 153, 150, 99, 255, 202, 89, 105, 77, 41, 89, 253, 130,
  27, 195, 134, 14, 66, 75, 24, 242, 7, 132, 234, 160, 203, 109, 195, 116,
  251, 144, 44, 28, 56, 231, 114, 50, 131, 185, 168, 138, 61, 35, 98, 78, 53
]);

const base58 = bs58.encode(wallet);
console.log(base58);


const input = prompt("Enter your name: ");

try {
  const wallet = bs58.decode(input);
  console.log(wallet);
} catch (e: any) {
  console.error("Invalid Base58 string:", e.message);
}
