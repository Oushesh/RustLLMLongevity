# Commands you will use everyday.

Command	                What it does
cargo new project_name	Creates a new Rust project
cargo build	            Compiles your project (in debug mode)
cargo run	            Builds & runs your project
cargo check	            Type-checks your code (super fast)
cargo test	            Runs your tests
cargo fmt	            Formats your code (like Prettier)
cargo clippy	        Lints & suggests idiomatic improvements


For your backend, we'll use cargo run to start the server, and cargo build --release when we want a production binary. 


It also searches for src/main.rs as entrypoint to the app.


8. Development order (build incrementally)

| Step | Goal                                                           | File(s)                      |
| ---- | -------------------------------------------------------------- | ---------------------------- |
| 1️⃣  | Setup `.env`, Docker Compose (Postgres + Qdrant)               | `.env`, `docker-compose.yml` |
| 2️⃣  | Implement `config.rs`, `main.rs`, `app.rs`, `routes/health.rs` | core                         |
| 3️⃣  | Add Prisma (`prisma/schema.prisma`), generate client           | `prisma/`                    |
| 4️⃣  | Implement `/chat` route (mock echo)                            | `routes/chat.rs`             |
| 5️⃣  | Implement `/search` (vector query)                             | `routes/search.rs`           |
| 6️⃣  | Add LLM calls (OpenAI embeddings + chat completion)            | `utils/llm.rs`               |
| 7️⃣  | Add observability (tracing, metrics)                           | `main.rs`, Docker            |
| 8️⃣  | Write tests and seed data                                      | `scripts/seed_data.rs`       |


Good Tabular Structure above. 