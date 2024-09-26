import { ClientEcashWallet, NostrClient } from "cashu_escrow_js";


const escrowWallet = await new ClientEcashWallet("http://localhost:3338");
console.log("After ClientEcashWallet");

const nostrClient = await new NostrClient("nsec182ul8zg2jlje6gtejs4pp4y4un674esq9qmrdxn2mewynkegahgqudmhvh");
console.log("After NostrClient");