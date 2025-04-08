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

- [ ] Proper error handling
- [ ] Handle multiple data types (Lists, HashMaps, Sets)
- [ ] Support for `EXPIRE` (key TTL)
- [ ] LRU Eviction Policies
- [ ] Pub/Sub
- [ ] Async version (Tokio based) — giga rizzler mode
- [ ] Benchmarks vs OG Redis for fun
- [ ] Dockerize it

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


---

Star it ⭐ | Fork it 🍴 | Play with it 🤹‍♂️

