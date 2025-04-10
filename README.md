# 🔥 rizzlerdb — Building Redis from scratch in Rust
<p align="center">
  <img src="./assets/banner.png" alt="Rusty Redis Banner" />
</p>

A learning project where I'm building my own Redis-like server (baby to giga chad levels) in Rust — fully from scratch, no external crates for the core logic. Just raw TCP, RESP protocol parsing, and pure violence.

## Features so far 🍃

- Multi-threaded TCP server (yes, ours is multi-threaded unlike OG Redis lmao)
- Basic RESP protocol parsing
- Commands implemented:
  - `PING`
  - `ECHO`
  - `SET key value`
  - `GET key`
  - `INCR key`
  - `INCRBY key amount`
  - `DECR key`
  - `DECRBY key amount`
  - `DEL key`
  - `EXISTS key`
  - `EXPIRE key seconds`
  - `TTL key`
  - `PERSIST key`
  - `KEYS pattern`
  - List commands: `LPUSH`, `RPUSH`, `LPOP`, `RPOP`, `LRANGE`
  - Hash commands: `HSET`, `HGET`, `HDEL`, `HGETALL`

## How to run 💻

```bash
cargo run
```

And in another terminal:

```bash
redis-cli -p 6379
```

Try out commands like:

```bash
> SET name gigachad
OK

> GET name
"gigachad"

> INCR counter
(integer) 1

> DEL name
(integer) 1
```

---

## What's cooking next? 🍳

### Phase 2 — The rizzler arc continues

- [ ] Modularization of code (time to stop writing sins)
- [ ] Pub/Sub support (real Redis influencer vibes)
- [ ] Sorted Sets (ZADD, ZSCORE, ZRANGE) maybe... idk if I’m that jobless
- [ ] RDB / AOF Persistence (so data stops ghosting us)
- [ ] Proper error handling everywhere (even for clowns)
- [ ] Config support (give user some power)
- [ ] Async version (Tokio powered ultra rizz)
- [ ] LRU / LFU Eviction Policies (memory discipline)
- [ ] Dockerize like a true backend bro
- [ ] Benchmarks vs OG Redis (for flex only)
- [ ] Custom CLI client (optional sauce)

---

## Why this project? 🤔

Redis is love. Rust is life. And building things from scratch teaches you more than 100 tutorials combined.

Plus, flex material for my resume & Twitter.

---

## Credits

Massive inspiration from:
- [redis.io docs](https://redis.io/docs/latest/)
- [RESP Protocol Spec](https://redis.io/docs/reference/protocol-spec/)

---

Star it ⭐ | Fork it 🍴 | Play with it 🤹‍♂️

