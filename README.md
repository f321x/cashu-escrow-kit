# Ecash escrow on Nostr concept

This project originated from the Ecash Hackathon 2024.  

## Idea
An escrow solution for trading projects (e.g. online shops) facilitating their payments over the Cashu ecash protocol. The trading parties can agree upon a escrow provider which is either hardcoded or can be discovered trough a Nostr announcement event. How the escrow provider is chosen depends on the software implementing the client library (e.g. reputation based ranking).  
Everyone can run an escrow coordinator and anounce their service publicly trough Nostr.  
The buying party locks its funds in a 2-of-3 P2PK ecash token which can then be unlocked by the buyer and seller (happy path) or the coordinator and one of the trading parties (escrow mediation path).  

This makes it possible to separate away the escrow provider from the trading plattform operator which can result in the follwing benefits for traders, developers and operators:

* Distributing trust between trading plattform operator and escrow operator
* Reducing operational burden of running a trading plattform
* Formation of an escrow provider market due to low entry barrier (driving down fees and favouring honest providers)
* Simple integration of escrow features in all kinds of trading plattforms and applications
* No vendor lock-in to a single large escrow provider necessary
* Safer trading conditions in low trust environments (e.g. pseudonymous traders on nostr- or onion markets)
* Good privacy for traders in happy case (coordinator has few, ephermal informations about trade and traders)  

## Architecture

![Screenshot from 2024-06-20 22-26-21](https://github.com/f321x/ecash-escrow-nostr-concept/assets/51097237/8b227061-da61-436d-bedc-0a1a25602b50)
