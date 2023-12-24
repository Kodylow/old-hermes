# Hermes: A Noncustodial Lightning Address Messenger w/Fedimint

Hermes is a non-custodial, async lightning address server using Fedimint Ecash on the backend.

Users register their nostr pubkey and username with the Hermes server to create a lightning address.

When a sender pays to the lightning address, Hermes uses a Fedimint Client to receive the Lightning Payment immediately by locking the  ecash to the user's registered pubkey.

Hermes then sends the created ecash notes to the recipient via Nostr DM.

You can register a public key and username on the server by paying a small fee in ecash. The server will then act as a LNURL Lightning Address server for you.

When someone attempts to pay you via your Lightning Address, the server will use your public key to create a lightning invoice and gateway transaction. 

The lightning gateway (separate from the Hermes Server) claims the invoice preimage by locking ecash to the registered public key. 

This immediately completes the lightning payment side of the transaction.

The Hermes server then sends you the locked ecash notes via DM, which you can redeem whenever you next connect. The Hermes server CANNOT spend your ecash, but it could maliciously not send it to you (leaving the server with some unspendable ecash).

## Running the Hermes Server

1. Clone the repo, make sure you have rust and docker installed.

2. Run `docker compose up -d` to start the postgres database. Make sure you change the username and password in the `docker-compose.yaml`

3. Set environment variables in `.env`, see `example.env` for reference
   
4. Run `cargo run` to start the hermes server.
