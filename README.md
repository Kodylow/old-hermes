<img src="assets/hermes_nostrich.png" width="500">

# Hermes: A Lightning Address Messenger w/Fedimint

Hermes is an asynchronous lightning address server that uses Fedimint Ecash on the backend. The protocol flow of Hermes is as follows:

**Note:** The Hermes server does not receive the ecash (**unless it's lying and giving its own invoices and not following the protocol at all, which it might be doing if malicious or compromised. It's a Lightning Address not magic.**), it is locked as a Fedimint Lightning Gateway Contract to the User's Pubkey. It doesn't even need to send you the notification, your Fedimint Client can scan for the payment on its own. The notification just makes the scan faster by telling the client to look for a specific payment or set of payments.

## Registration

1. Users register their Nostr public key and username with the Hermes server. This registration process creates a lightning address for the user.

2. The registration requires a small fee in ecash or lightning.

## Receiving Payments

1. Sender follows normal lnurlp protocol hitting well-known and callback endpoints.

2. Hermes server creates a Fedimint Lightning Gateway transaction based off the receiver's public key, and returns the invoice to the sender via the callback endpoint.

3. Sender pays the lightning invoice, which the lightning gateway immediately completes by locking ecash to the receiver's public key.

4. Hermes server sends a notification to the receiver that they have received a payment.

5. When the receiver's Fedimint Client next connects to their federation, they scan for the payment and reissue the pubkey locked ecash.

Current implementation is a bit different (Hermes receives the ecash and sends the notes to you), but the above is the goal to be purely a messenger vs a passthrough custodian and is pending a few changes to the Fedimint Client.

## Running the Hermes Server

1. Clone the repository and ensure that Rust and Docker are installed on your system.

2. Start the Postgres database by running `docker compose up -d`. Remember to change the username and password in the `docker-compose.yaml` file.

3. Set the environment variables in the `.env` file. Refer to `example.env` for guidance.

4. Start the Hermes server by running `cargo run`.
