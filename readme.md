# Reverse Shell in Rust
- Starta klienten först: `cargo run --bin client` Den lyssnar på 127.0.0.1:4444.
- Starta servern i ett annat terminalfönster: `cargo run --bin server` Den ansluter bakåt till klienten.
- Skriv kommandon i klientens terminal (t.ex. whoami), så körs de på servern och resultatet skickas tillbaka.

Gnu behövs för att bygga, går att ändra i `.cargo\config.toml`