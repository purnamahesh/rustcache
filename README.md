# rustcache

> **Project 1 of 4** | Prereqs: None (this is where you start)
> Sequence: **rustcache** → logforger → plugsmith → ironweave

An in-memory key-value cache with TTL expiration and LRU eviction, built in Rust.

Think `memcached` but as a Rust library with a CLI interface. Can be embedded in other Rust projects or run standalone accepting commands via stdin or TCP.

## Why This Project Exists

This is a learning project designed to teach Rust's ownership, borrowing, and the borrow checker by building something real and useful. Every phase introduces a new ownership challenge that the cache domain naturally demands:

- **Storing values**: who owns the cached data?
- **Reading values**: how to return references without moving data out?
- **Updating values**: exclusive mutable access while others might be reading?
- **Evicting entries**: how to remove data safely when references might exist?
- **Sharing cache across threads**: Arc, Mutex, RwLock decisions
- **Entry API pattern**: the borrow checker's most famous puzzle
- **Interior mutability**: when the borrow checker's rules feel too strict

## Architecture

```
┌──────────────────────────────────┐
│         CLI / TCP Interface       │
│   (commands: GET, SET, DEL, TTL) │
└──────────┬───────────────────────┘
           │
┌──────────▼───────────────────────┐
│        Cache Engine               │
│  ┌─────────────┐ ┌────────────┐  │
│  │  HashMap     │ │ LRU List   │  │
│  │  (K → Entry) │ │ (ordering) │  │
│  └─────────────┘ └────────────┘  │
│  ┌─────────────┐ ┌────────────┐  │
│  │  TTL Tracker │ │ Stats      │  │
│  └─────────────┘ └────────────┘  │
└──────────────────────────────────┘
```

---

## Phase 1: The Basics - Own Your Data

**Goal**: Build a basic key-value store that compiles and handles ownership correctly.

**Rust Concepts Covered**: Ownership, Move semantics, Clone vs Copy, String vs &str, References (`&` and `&mut`), Dereferencing (`*`), HashMap basics, Option\<T\>

### What To Build

- A cache struct that OWNS a HashMap of String keys to String values. Think about what "owns" means here -- the struct is responsible for the lifetime of all the data inside it.
- Three core methods:
  - One that stores a key-value pair. Hint: the method should take ownership of both the key and the value. What type should the parameters be?
  - One that retrieves a value by key. Hint: should this return the value itself (moving it out of the cache) or let the caller look at it temporarily? Think about what the caller wants -- to read or to take. You'll use `&` to create a reference. When the caller has that reference, they may need `*` to dereference it (e.g., to compare the value or copy it out).
  - One that removes a key-value pair. Hint: unlike retrieval, deletion genuinely removes data. What should it return so the caller knows what was deleted?
- A simple REPL loop that reads commands from stdin and dispatches them.
  - You will immediately hit "cannot move out of borrowed content" or "value borrowed after move" when parsing command strings. THIS IS THE LEARNING.
  - Hint: when you split a string into parts, think about who owns the original string and what the parts borrow from.
  - When matching on `Option<&V>` from HashMap::get, you'll encounter `ref` in patterns — this lets you bind by reference instead of by move. Understand why `Some(val)` vs `Some(ref val)` matters.

### Questions To Answer Before Coding

1. What is the difference between `String` and `&str`? When do you use each?
2. If you have a `String` variable and pass it to a function, can you use that variable afterward?
3. What does HashMap's insertion method return, and why does that return value matter?
4. How do you look up a value in a HashMap without taking ownership of it? What does the `&` in front of the key do?
5. What is the difference between a HashMap method that gives you a reference to a value vs one that removes and gives you the value?
6. When you have `&String`, how does Rust automatically coerce it to `&str`? (Hint: the `Deref` trait)
7. What does `*` do when applied to a reference? When do you need it vs when does Rust auto-deref for you?

### End Expectations

- `SET name "Alice"` stores the value
- `GET name` prints "Alice" (the value stays in the cache -- it was not moved out)
- `GET missing` prints "(nil)"
- `DEL name` removes and prints the old value
- No `.clone()` unless you can explain exactly why it is needed
- Code compiles without any `unsafe`

---

## Phase 2: Rich Values & Entry API

**Goal**: Support multiple value types and learn the Entry API -- the borrow checker's favorite puzzle.

**Rust Concepts Covered**: Enums as tagged unions, Pattern matching with ownership (`ref`, `ref mut` in patterns), Dereferencing with `*` to modify through `&mut`, The Entry API (occupied/vacant), Mutable references and reborrowing, `std::mem::replace`

### What To Build

- Replace plain String values with an enum that can hold: a String, an Integer (i64), a List (Vec of Strings), or a nested HashMap. Hint: you will need to define an enum with four variants, each holding a different type.
- Implement `INCR key` -- increment an integer value, or initialize to 1 if missing.
  - This is where you will fight the borrow checker the hardest. You need to: check if a key exists (which requires an immutable borrow of the map), then either update the value (which requires a mutable borrow) or insert a new one (also mutable). You cannot hold both borrows at the same time.
  - The Entry API is the solution. Understand WHY it exists -- it is a borrow checker workaround baked into the standard library. It gives you a handle that represents "this slot in the map" so you only borrow once.
  - When you get a mutable reference to the value via Entry, you'll use `*` to dereference and modify it. For example, once you have `&mut i64`, you need `*value += 1` — the `*` reaches through the reference to the actual data.
- Implement `LPUSH key value` -- push a value to the front of a list, creating the list if it does not exist. Hint: same Entry API pattern.
- Implement `LRANGE key start stop` -- return a slice of the list.
  - Think about what happens when you return a reference to data inside the cache. What lifetime does that reference have? How long is the caller allowed to hold onto it?
- Implement `TYPE key` -- return the type name of the value stored at a key.

### Questions To Answer Before Coding

1. Why can you not do "get_mut to check, else insert"? Draw the borrow lifetimes on paper. When does the first borrow end? When does the second begin?
2. What is `or_insert_with` on the Entry API, and how does it solve the double-borrow problem?
3. When you pattern match on an enum, does it move or borrow the inner data? How do `ref` and `ref mut` in patterns control this? What about matching on `&value` vs `value`?
4. What does `std::mem::replace` do, and why is it useful for swapping values in-place without violating borrowing rules?

### End Expectations

- `SET count 0` then `INCR count` five times, then `GET count` returns "5"
- `LPUSH colors red` then `LPUSH colors blue`, then `LRANGE colors 0 -1` returns ["blue", "red"]
- `TYPE count` returns "integer", `TYPE colors` returns "list"
- Zero uses of `unwrap()` in library code. All errors handled properly.
- Entry API used correctly -- not worked around with clone-then-double-lookup

---

## Phase 3: TTL & Expiration - Taming Shared Mutability

**Goal**: Add time-to-live to entries. Learn interior mutability and when the borrow checker's normal rules are not enough.

**Rust Concepts Covered**: Interior mutability (RefCell, Cell), Rc\<RefCell\<T\>\> pattern, When to use Cell vs RefCell vs Mutex, Newtype pattern for type safety, `Instant` and `Duration` for time

### What To Build

- Add an expiration timestamp to each cache entry. Hint: not all keys expire, so this should be optional. You will want a struct that wraps the value alongside optional expiration metadata.
- `SET key value EX seconds` -- set a value with an expiration time.
- `TTL key` -- returns seconds remaining, -1 if no expiry, -2 if key does not exist.
- `EXPIRE key seconds` -- add expiration to an existing key.
- **Passive expiration**: check if a key is expired on `GET` (lazy cleanup). Hint: your get method now needs to potentially mutate the map even though "get" sounds like a read-only operation. This is your first encounter with the "I want to mutate through a shared reference" problem.
- **Active expiration**: periodically scan for and remove expired entries.
  - Problem: you need to iterate the map (immutable borrow) and remove entries (mutable borrow) at the same time. The borrow checker will not allow this.
  - Hints: explore three approaches. (1) Collect the expired keys into a Vec first, then remove them in a second pass. (2) Use `retain()` which handles the borrowing internally. (3) Interior mutability. Think about which is simplest and which is most efficient.
- Track cache statistics (hits, misses, evictions). Hint: this is a great use case for `Cell<u64>` -- you want to increment counters even through shared `&self` references, and Cell works for Copy types like integers without runtime borrow checking overhead.

### Questions To Answer Before Coding

1. Why can you not modify a HashMap while iterating over it? What could go wrong at a memory level?
2. What is the difference between `Cell` and `RefCell`? When would you use each?
3. If you have `&self` (a shared reference), how can you still mutate a `Cell<u64>` inside the struct?
4. What does `RefCell` give you that `Cell` cannot? What is the runtime cost?
5. What happens if you call `.borrow_mut()` on a `RefCell` that is already borrowed? Why is this a problem that does not exist with compile-time borrows?

### End Expectations

- `SET session abc123 EX 60` stores a key that expires after 60 seconds
- `TTL session` shows remaining seconds
- `GET session` after expiry returns "(nil)" and the key is cleaned up
- `STATS` command shows hits, misses, and expired keys count
- Active cleanup runs without panicking (no double-borrow runtime errors from RefCell)
- Expired keys do not leak memory

---

## Phase 4: LRU Eviction - Data Structures & Ownership

**Goal**: Implement LRU eviction with a max-capacity cache. Confront the hardest ownership challenge: a data structure with multiple access paths to the same data.

**Rust Concepts Covered**: Doubly-linked list ownership problem (why Rust makes this hard), `unsafe` as a last resort (and why you should avoid it), Alternative: using indices instead of pointers, `VecDeque` vs hand-rolled linked list, The `Box` smart pointer for heap allocation, `Deref` and `DerefMut` (how `Box<T>` auto-derefs to `T`)

### What To Build

- Set a maximum number of entries the cache can hold.
- When the cache is full and a new entry is added, evict the least recently used entry.
- Every `GET` should mark the accessed entry as "recently used."
- LRU typically needs a doubly-linked list combined with a HashMap. In Rust, this is notoriously hard because:
  - Each node in a doubly-linked list has TWO owners (the previous node's "next" pointer and the next node's "prev" pointer). Rust's ownership model says each value has exactly one owner.
  - You could use `Rc<RefCell<Node>>` but it is clunky, slow, and risks memory leaks from reference cycles.
  - The pragmatic solution: use a Vec (or VecDeque) with indices instead of pointers. Each "pointer" is just a usize index into the Vec. This is a REAL Rust pattern called "arena allocation by index." It sidesteps the ownership problem entirely because the Vec owns all the nodes and indices are just numbers.
- Implement `MAXMEMORY n` to set the maximum capacity.
- Implement `EVICT` to manually trigger eviction of the least recently used entry.
- Implement `DBSIZE` to show the current entry count vs the maximum.

### Questions To Answer Before Coding

1. Why is a doubly-linked list considered "Rust's hardest beginner data structure"? What specific ownership rule does it violate?
2. How does using indices into a Vec solve the ownership problem that pointers create?
3. What is the "generational index" pattern and why does it prevent use-after-free bugs even with index-based structures?
4. If you use `Rc<RefCell<Node>>` for a linked list, what is the risk of memory leaks? Why won't the drop mechanism save you?
5. Why does the standard library's `LinkedList` exist but is rarely recommended?
6. When you have `Box<Node>`, why can you call methods on `Node` directly? What is the `Deref` trait doing behind the scenes? What about `DerefMut` for mutable access?

### End Expectations

- `MAXMEMORY 100` sets the cache to hold at most 100 entries
- Adding entry 101 evicts the least recently used entry
- `GET` on an entry moves it to "most recently used"
- `INFO` shows: total entries, max capacity, total evictions
- LRU eviction is O(1) for both access and eviction
- No `unsafe` code (use the index-based approach)
- Benchmark: can handle 100k SET operations in under 1 second

---

## Phase 5: Thread Safety & Concurrent Access

**Goal**: Make the cache safe to use from multiple threads. Understand Send, Sync, and the shared-state concurrency model.

**Rust Concepts Covered**: Arc\<T\> for shared ownership across threads, Mutex\<T\> vs RwLock\<T\> tradeoffs, Send and Sync traits (the compiler enforces thread safety!), Lock granularity (one big lock vs sharded locks), Poisoned locks and recovery, `parking_lot` as a better Mutex

### What To Build

- Wrap the cache so multiple threads can access it concurrently. Hint: you will need a combination of a reference-counting smart pointer and a lock. Think about which reference-counting pointer is safe to send across threads, and which lock type matches your read-heavy workload.
- TCP server that accepts connections and processes commands. Hint: each connection can run on its own thread. Think about what data needs to be shared and what can be per-connection.
- Think about lock granularity:
  - Do you need exclusive access (Mutex) or many-readers/one-writer (RwLock)?
  - Most cache workloads are read-heavy, so RwLock seems better. But RwLock has writer starvation issues and higher overhead. Benchmark both.
  - Advanced: shard the cache into N partitions, each with its own lock. A key's shard is determined by hashing. This means operations on different keys never contend.
- Implement `MULTI` / `EXEC` for atomic batch operations. Hint: this needs an exclusive lock held for the entire duration of the batch, which means no other operations can interleave.
- The compiler will REFUSE to let you share non-Send types across threads. This is Rust's superpower -- data race prevention at compile time.

### Questions To Answer Before Coding

1. What does `Arc` do that `Rc` does not? Why can you not use `Rc` across threads?
2. What is the difference between `Mutex<T>` and `RwLock<T>`? When is each the better choice?
3. What does it mean for a type to be `Send`? What about `Sync`? How do these two traits relate?
4. If you have `Arc<Mutex<Cache>>`, which operations require locking and which do not?
5. What is lock poisoning and how should you handle it in a cache server?
6. What is "sharded locking" and why does it improve throughput compared to a single global lock?

### End Expectations

- Multiple clients can connect via TCP simultaneously
- Reads do not block other reads (via RwLock or sharding)
- Writes are atomic and consistent
- `MULTI/EXEC` provides transaction-like batching
- No data races (the compiler ensures this)
- Benchmark: 50k ops/sec with 10 concurrent clients
- Passes `cargo test` with `--test-threads=1` AND the default thread count

---

## Phase 6: Production Polish

**Goal**: Make it a real tool someone would actually use.

**Rust Concepts Covered**: Error handling with `thiserror` and `anyhow`, Builder pattern, `From`/`Into` conversions, `AsRef`/`AsMut` for flexible function parameters, `Borrow` trait (how HashMap lookups work with owned vs borrowed keys), `Display` trait for user-facing output, `clap` for CLI argument parsing, Serialization with `serde` for persistence

### What To Build

- Proper CLI with `clap`: `rustcache --port 6380 --maxmemory 10000 --persist snapshot.rdb`
- Make your `get` method accept `impl AsRef<str>` instead of just `&str` — this lets callers pass `String`, `&str`, or `&String` without caring. Understand the difference between `AsRef`, `Borrow`, and plain references.
- Understand why HashMap can look up a `String` key with a `&str` — the `Borrow` trait makes this work. Implement similar flexibility in your own API.
- Snapshot persistence: periodically serialize the cache to disk, restore on startup. Hint: you will need to think about how to serialize your value enum -- `serde` with derive macros makes this straightforward.
- Configuration file support (TOML).
- Structured logging with `tracing`.
- Graceful shutdown: catch SIGINT, flush data to disk before exiting. Hint: look into signal handling and how to notify your server loop to stop accepting connections.
- Comprehensive test suite: unit tests for cache logic, integration tests for the TCP protocol.

### End Expectations

- `cargo install` produces a working binary
- Starts, serves requests, handles shutdown gracefully
- Persists data across restarts
- `--help` prints usage information
- Error messages are helpful, not panics
- Greater than 80% test coverage on core cache logic

---

## Project Structure

```
rustcache/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs           # CLI entry point, TCP server
│   ├── lib.rs            # Public API (use as library)
│   ├── cache.rs          # Core cache engine
│   ├── entry.rs          # Cache entry (value + metadata)
│   ├── value.rs          # Value enum (String, Int, List, Map)
│   ├── lru.rs            # LRU eviction tracker
│   ├── ttl.rs            # TTL/expiration manager
│   ├── protocol.rs       # Command parsing (RESP-like)
│   ├── server.rs         # TCP server (Phase 5)
│   ├── config.rs         # Configuration
│   ├── persistence.rs    # Snapshot save/restore (Phase 6)
│   └── error.rs          # Error types
├── tests/
│   ├── cache_tests.rs
│   ├── lru_tests.rs
│   ├── protocol_tests.rs
│   └── integration.rs
└── benches/
    └── cache_bench.rs
```

---

## The Ownership Journey

This is the conceptual arc you will travel through:

- **Phase 1**: "I can't return a reference to data inside my struct??" -- Understanding borrows and why they have lifetimes tied to the data they point to.
- **Phase 2**: "I need to check-then-insert but the borrow checker won't let me!" -- The Entry API and why the standard library provides it.
- **Phase 3**: "I want to mutate through a shared reference??" -- Interior mutability and the runtime borrow checking tradeoff.
- **Phase 4**: "A linked list has TWO pointers to each node??" -- Index-based data structures and thinking in arenas instead of pointers.
- **Phase 5**: "The compiler won't let me share this across threads!" -- Send, Sync, Arc, and Mutex as the foundation of safe concurrency.
- **Phase 6**: "I can build real things in Rust without fighting the compiler." -- Fluency.

---

## Common Pitfalls

Read these AFTER you get stuck. Getting the error and struggling with it first is how the concepts stick.

1. **"cannot borrow as mutable because it is also borrowed as immutable"** -- You are trying to read and write the HashMap at the same time. The Entry API exists for exactly this situation.

2. **"value does not live long enough"** -- You are trying to return a reference to something that will be dropped. Consider returning a clone, or restructuring so the data lives long enough for the reference to be valid.

3. **"cannot move out of borrowed content"** -- You are trying to take ownership of something you only have a reference to. Use `.clone()`, `.to_owned()`, or rethink your API so it does not need ownership.

4. **"type doesn't implement Copy"** -- Strings and Vecs cannot be implicitly copied because they manage heap memory. You need `.clone()` or to pass by reference instead.

5. **".clone() everywhere and it feels wrong"** -- It probably is. Each clone is a signal you might need to rethink ownership. But sometimes clone IS the right answer -- the key is being able to explain why.

6. **"I need Rc\<RefCell\<T\>\> for everything"** -- Step back. Usually there is a simpler design. Index-based approaches often work better and are faster.

7. **"type `&T` doesn't implement Display" (or similar)** -- You have a reference but need the value. Use `*` to dereference, or Rust's auto-deref will often handle it. Understand when auto-deref kicks in (method calls, comparison operators) vs when you need explicit `*`.

8. **Confused by `&`, `*`, `ref`, and `&mut`** -- Think of it this way: `&` creates a reference (pointer), `*` follows a reference (dereferences), `ref` in a pattern binds by reference instead of by move, and `&mut` creates an exclusive mutable reference. When you have `&mut value` and want to change it, you need `*value = new_thing`.

---

## Key Dependencies

```toml
# Phase 1-4 (minimal dependencies - that's the point)
[dependencies]

# Phase 5
# (just std library threads, or optionally:)
# parking_lot = "0.12"  # Better Mutex/RwLock

# Phase 6
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

Phases 1 through 4 intentionally use NO external dependencies. The whole point is learning standard library fundamentals.

---

## Success Criteria

By completing this project you will:

- Never be confused by "cannot borrow as mutable" errors again
- Fluently use `&` (reference), `&mut` (mutable reference), `*` (dereference), `ref`/`ref mut` (pattern binding), and auto-deref
- Understand `Deref`/`DerefMut` traits and how they power smart pointers and auto-coercion
- Know `AsRef`, `AsMut`, and `Borrow` for writing flexible APIs
- Understand when to use `&`, `&mut`, owned values, Clone, Copy, Rc, and Arc
- Know when and why to use the Entry API, Cell, RefCell, Mutex, and RwLock
- Be able to design data structures that work WITH the borrow checker instead of against it
- Understand why Rust's ownership model prevents entire categories of bugs that plague C, C++, and even garbage-collected languages
- Have a useful, fast, thread-safe cache you can embed in other projects or run as a standalone server
