import { ClientEcashWallet, NostrClient, TradeContract, TradeNostrIdentities, EcashIdentities, InitEscrowClient, TradeMode } from "cashu_escrow_js";

async function createWallet() {
    console.log("Creating wallet...");
    const escrowWallet = await new ClientEcashWallet("http://localhost:3338");
    console.log("returning wallet created...");
    return escrowWallet;
}

async function runTradePipeline(role, escrowWallet, partnerPubkey) {
    const mode = role === "buyer" ? TradeMode.Buyer : TradeMode.Seller;
    
    if (mode == TradeMode.Buyer) {
        console.log("minting some ECash...");
        const token = await escrowWallet.mint(BigInt(5000));
        console.log("minted token: ", token);
    }

    const description = "Test contract";
    const amount = BigInt(5000);
    const buyerNpub = "npub1pjnvp4kwud0r80kk23h726tn8ewfqmd4y5m9g2rggs0xrdzrgexq0hv5gr";
    const sellerNpub = "npub1rsdt7jfwkk9mnzn6vne6geuaaz6rvtr6twxsedzrkan3jzk2uyjsczwcxu";
    const coordinatorNpub = "npub1hcsc4r3xc9ygnefp4eqyan9r46tjvd3w0dxk2hgydc9k6m5xd3jq2hkjqp";
    const tradeNostrIdentities = new TradeNostrIdentities(buyerNpub, sellerNpub, coordinatorNpub);
    const timeLimit = BigInt(3 * 24 * 60 * 60);
    const {buyerEcashPubkey, sellerEcashPubkey} = mode === TradeMode.Buyer ? 
        {buyerEcashPubkey: escrowWallet.tradePublicKey, sellerEcashPubkey: partnerPubkey} : 
        {buyerEcashPubkey: partnerPubkey, sellerEcashPubkey: escrowWallet.tradePublicKey};
    
    console.log("Creating a TradeContract...");
    const ecashIdentities = new EcashIdentities(buyerEcashPubkey, sellerEcashPubkey);
    const tradeContract = new TradeContract(
        description,
        amount,
        tradeNostrIdentities,
        timeLimit,
        ecashIdentities,
    );
    console.log("Created a TradeContract", tradeContract);

    // NostrClient
    const nsec = mode == TradeMode.Buyer ? "nsec182ul8zg2jlje6gtejs4pp4y4un674esq9qmrdxn2mewynkegahgqudmhvh" : "nsec1vackt9cn8ujwz9t2yj6x29d4tgjx3uhp5h0fyev8tp5lnw40lcls3rp7hp";
    const relays = ["ws://localhost:4736"];
    const nostrClient = await new NostrClient(nsec, relays);
    console.log("After NostrClient.");

    // Init Escrow Client
    const initEscrowClient = await new InitEscrowClient(nostrClient, escrowWallet, tradeContract, mode);
    console.log("After InitEscrowClient.");

    // Trade Pipeline
    console.log("Starting trade pipeline...");
    const registeredClient = await initEscrowClient.registerTrade();
    console.log("After registerTrade.");
    const exchangedClient = await registeredClient.exchangeTradeToken();
    console.log("After exchangeTradeToken.");
    await exchangedClient.doYourTradeDuties();
    console.log("After doYourTradeDuties.");
}

export { createWallet, runTradePipeline };