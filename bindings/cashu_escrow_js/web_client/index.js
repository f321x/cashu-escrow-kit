import { ClientEcashWallet, NostrClient, TradeContract } from "cashu_escrow_js";


const escrowWallet = await new ClientEcashWallet("http://localhost:3338");
console.log("After ClientEcashWallet");

console.log("minting some ECash...");
const quoteId = await escrowWallet.mintQuote(BigInt(5000));
console.log("minted with quoteId:", quoteId);

console.log("Creating a TradeContract...");
const description = "Test contract";
const amount = BigInt(5000);
const buyerNpub = "npub1pjnvp4kwud0r80kk23h726tn8ewfqmd4y5m9g2rggs0xrdzrgexq0hv5gr";
const sellerNpub = "npub1rsdt7jfwkk9mnzn6vne6geuaaz6rvtr6twxsedzrkan3jzk2uyjsczwcxu";
const coordinatorNpub = "npub1hcsc4r3xc9ygnefp4eqyan9r46tjvd3w0dxk2hgydc9k6m5xd3jq2hkjqp";
const timeLimit = BigInt(3 * 24 * 60 * 60);
const buyerTradePubkey = escrowWallet.tradePublicKey;
const sellerTradePubkey = "to be replaced with a real trade public key";
const tradeContract = await new TradeContract(
    description,
    amount,
    buyerNpub,
    sellerNpub,
    coordinatorNpub,
    timeLimit,
    buyerTradePubkey,
    sellerTradePubkey,
);
console.log("Created a TradeContract:", tradeContract);

// NostrClient
const nostrClient = await new NostrClient("nsec182ul8zg2jlje6gtejs4pp4y4un674esq9qmrdxn2mewynkegahgqudmhvh");
console.log("After NostrClient");