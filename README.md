# Fedimint LNURL Server

This is a simple LNURL address server implementation using the Fedimint client.

You can register a public key and username on the server by paying a small fee in ecash. The server will then act as a LNURL Lightning Address server for you.

When someone attempts to pay you via your Lightning Address, the server will use your public key to create a lightning invoice and gateway transaction. 

The lightning gateway claims the invoice preimage by locking ecash to the registered public key.

When your fedimint client reconnects, it will claim the ecash received via the lightning address server as part of its normal sync process.
