# Word of Wisdom Proof-of-Work

The repository contains both a server and a client

The server performs a DDOS protection with the PoW algorithm
It uses the Hashcash implementation, the same as bitcoin uses,
It is pretty simple and well-known, it has large adoption
And very high security levels in comparison to others

Once the client provides a correct solution,
The server provides him with randomly chosen citate from the
word of wisdom book

## How to run

```bash
docker-compose up -d
docker-compose run client
```
