# 🔥 rizzlerdb — Building Redis from Scratch in Rust (Async + Pub/Sub Powered)

<p align="center">
  <img src="./assets/banner.png" alt="rizzlerdb logo" />
</p>

A blazing-fast, from-scratch Redis-like server written in Rust.

No frameworks. No shortcuts. Just raw TCP, manual RESP parsing, and pure backend villainy.
Now powered by Tokio for async handling and Pub/Sub support.

---

🚀 **Architecture Overview**

- 🧩 Clients connect via raw TCP on port `6379`
- 🧵 Each connection is handled asynchronously with Tokio tasks
- 🧠 Incoming commands are parsed using a hand-written RESP protocol parser
- 🧱 All data lives in a central `Database` struct guarded by `Arc<Mutex<_>>`
- 💾 For mutating ops, persistence is triggered using a `Persister` trait
  - Current: `JsonPersister` (writes state to `db.json`)
- 🔄 On boot, the server attempts to hydrate itself from disk
- 📣 Pub/Sub support with real-time message broadcasting via channels

---

✅ **Features Implemented**

### ☑ Core Infra

- [x] Async server with Tokio
- [x] Multithreaded + async TCP socket handling
- [x] Manual RESP parser (no dependencies)
- [x] In-memory store using `HashMap`, `Vec`, and friends
- [x] Auto persistence using `JsonPersister`
- [x] Boot-time state restoration from disk
- [x] Pub/Sub system with channel subscriptions and async broadcasts

### ☑ Command Support

#### 🧠 String Ops

`PING`, `ECHO`, `SET`, `GET`, `DEL`, `EXISTS`, `INCR`, `INCRBY`, `DECR`, `DECRBY`

#### ⏳ Expiry & TTL

`EXPIRE`, `TTL`, `PERSIST`

#### 🧺 List Ops

`LPUSH`, `RPUSH`, `LPOP`, `RPOP`, `LRANGE`, `LLEN`, `LINDEX`, `LSET`

#### 📐 Set Ops

`SADD`, `SREM`, `SMEMBERS`, `SISMEMBER`, `SCARD`

#### 🗃️ Hash Ops

`HSET`, `HGET`, `HDEL`, `HKEYS`, `HVALS`, `HGETALL`, `HEXISTS`, `HLEN`

#### 📡 Pub/Sub

`PUBLISH`, `SUBSCRIBE` — real-time message delivery between clients

#### 🔍 Other Goodies

`KEYS` with basic globbing/pattern matching

---

📂 **Run It Locally**

```bash
cargo run
```

In another terminal:

```bash
redis-cli -p 6379
```

Sample commands:

```redis
> SET name gigachad
> GET name
> INCR count
> LPUSH queue task1
> HSET user name yash
> SUBSCRIBE news
> PUBLISH news "the backend villain strikes again"
```

---

🔮 **Next Phase**

What’s next in the evolution of the RizzlerDB:

- [ ] Background expiry cleanup workers
- [ ] Full codebase refactor & modularization
- [ ] More robust Pub/Sub support (multi-channel, unsubscribe, patterns)
- [ ] Config file support (custom port, persistence toggle, etc.)
- [ ] LRU / LFU eviction strategies
- [ ] RDB-style snapshotting (save memory state)
- [ ] AOF-style persistence (append-only file)
- [ ] Docker support (containerized deployment)
- [ ] Performance benchmarks vs Redis (because why not?)
- [ ] Build a custom CLI client (no more redis-cli dependency)
- [ ] Add logging, telemetry, and proper error handling
- [ ] CI/CD pipeline + GitHub Actions

---

🤔 **Why This Project?**

Redis is the godfather of in-memory databases. Rust is the language of backend legends. Combine both and you’re forced to learn the internals, system design, memory management, concurrency models, protocol parsing, and async runtime behavior.

This isn’t just a project. It’s an origin story. A backend villain arc in full swing.

---

📎 **Repo**

GitHub: [github.com/pixperk/redis_in_rust](https://github.com/pixperk/redis_in_rust)

---

🙏 **Credits**

- Redis Official Docs
- RESP Protocol Spec
- Tokio and Rustacean Community

---

Star it ⭐ | Fork it 🍴 | Hack it 🧠 | Rizz it 🦝 | Deploy it 💥

