import { ClientEcashWallet, NostrClient, TradeContract, TradeNostrIdentities, EcashIdentities, InitEscrowClient, TradeMode } from "cashu_escrow_js";


const escrowWallet = await new ClientEcashWallet("http://localhost:3338");
console.log("After ClientEcashWallet");

console.log("minting some ECash...");
const quoteId = await escrowWallet.mintQuote(BigInt(5000));
console.log("minted with quoteId:", quoteId);

const description = "Test contract";
const amount = BigInt(5000);
const buyerNpub = "npub1pjnvp4kwud0r80kk23h726tn8ewfqmd4y5m9g2rggs0xrdzrgexq0hv5gr";
const sellerNpub = "npub1rsdt7jfwkk9mnzn6vne6geuaaz6rvtr6twxsedzrkan3jzk2uyjsczwcxu";
const coordinatorNpub = "npub1hcsc4r3xc9ygnefp4eqyan9r46tjvd3w0dxk2hgydc9k6m5xd3jq2hkjqp";
const tradeNostrIdentities = new TradeNostrIdentities(buyerNpub, sellerNpub, coordinatorNpub);
const timeLimit = BigInt(3 * 24 * 60 * 60);
const buyerEcashPubkey = escrowWallet.tradePublicKey;
const sellerEcashPubkey = "to be replaced with a real trade public key";
console.log("Creating a TradeContract...");
const ecashIdentities = new EcashIdentities(buyerEcashPubkey, sellerEcashPubkey);
const tradeContract = await new TradeContract(
    description,
    amount,
    tradeNostrIdentities,
    timeLimit,
    ecashIdentities,
);
console.log("Created a TradeContract:", tradeContract);

// NostrClient
const nsec = "nsec182ul8zg2jlje6gtejs4pp4y4un674esq9qmrdxn2mewynkegahgqudmhvh";
const relays = ["ws://localhost:4736"];
const nostrClient = await new NostrClient(nsec, relays);
console.log("After NostrClient");

// Init Escrow Client
const initEscrowClient = await new InitEscrowClient(nostrClient, escrowWallet, tradeContract, TradeMode.Buyer);
console.log("After InitEscrowClient");

// Trade Pipeline
initEscrowClient.registerTrade();
console.log("After registerTrade");