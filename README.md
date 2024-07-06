# Ecash escrow on Nostr concept

This project originated from the [Ecash Hackathon 2024](https://web.archive.org/web/20240527181133/https://www.nobsbitcoin.com/ecash-hackday-v2-to-take-place-in-berlin-on-june-20-21/). 

## Idea
An escrow solution for trading projects (e.g. online shops) facilitating their payments over the [Cashu ecash protocol](https://cashu.space/). The trading parties can agree upon an escrow provider which is either hardcoded or can be discovered through a [Nostr](https://nostr.com/) announcement [event](https://github.com/nostr-protocol/nips/blob/master/01.md). How the escrow provider is chosen depends on the software implementing the client library (e.g. reputation based ranking).  
Everyone can run an escrow coordinator and announce their service publicly trough Nostr.  
The buying party locks its funds in a [2-of-3 P2PK ecash token](https://github.com/cashubtc/nuts/blob/main/11.md) which can then be unlocked by the buyer and seller (happy path) or the coordinator and one of the trading parties (escrow mediation path).  

This makes it possible to separate away the escrow provider from the trading plattform operator which can result in the following benefits for traders, developers and operators:

* Distributing trust between trading plattform operator and escrow operator
* Reducing operational burden of running a trading platform
* Formation of an escrow provider market due to low entry barrier (driving down fees and favouring honest providers)
* Simple integration of escrow features in all kinds of trading plattforms and applications
* No vendor lock-in to a single large escrow provider necessary
* Safer trading conditions in low trust environments (e.g. pseudonymous traders on nostr- or onion markets)
* Good privacy for traders in happy case (coordinator has few, ephemeral informations about trade and traders)  

## Architecture

![Screenshot from 2024-06-20 22-26-21](https://github.com/f321x/ecash-escrow-nostr-concept/assets/51097237/8b227061-da61-436d-bedc-0a1a25602b50)

#### Additions and thoughts

##### Submitting escrow conditions   
Both trading parties have to commit to their trade obligations to the coordinator. This commitment has to contain all information necessary for the coordinator to decide which trade party fulfilled their obligations in the case of an escrow mediation. This can include payout information, amounts, timeframes and a freely written trade contract. When possible, information can be submitted as hash to improve privacy against the coordinator.

##### Nostr communication
To reduce uneccesary burden on relays we can aim to use ephemeral event types for communication between traders and coordinator.

##### Client
The client could be distributed as wasm library and rust crate. There could also be a compilation flag that decides if the client gets built with nostr communication logic or only with nostr event creation logic. First would be useful for inclusion in traditional trading platforms and second would be useful for nostr based trading platforms already including relay/communication logic.

## Acknowledgments
Special thanks to the following projects, without them this project wouldn't be possible:

* [Cashu Development Kit](https://github.com/cashubtc/cdk)
* [Rust Nostr](https://github.com/rust-nostr/nostr)

## Contribution
If you want to discuss this project or contribute feel free to join the [SimpleX messenger group](https://simplex.chat/contact#/?v=2-5&smp=smp%3A%2F%2F6iIcWT_dF2zN_w5xzZEY7HI2Prbh3ldP07YTyDexPjE%3D%40smp10.simplex.im%2FXp-lzznxmQTAKO3yJQtx_Bu9j2ZxDmRS%23%2F%3Fv%3D1-2%26dh%3DMCowBQYDK2VuAyEATACuD83g5rq9Eooa7-tv0q1vff8HUs8ucJ0OgSJ36zQ%253D%26srv%3Drb2pbttocvnbrngnwziclp2f4ckjq65kebafws6g4hy22cdaiv5dwjqd.onion&data=%7B%22type%22%3A%22group%22%2C%22groupLinkId%22%3A%22Oe7Ff4nsqtAjx4sVV8rcDA%3D%3D%22%7D)