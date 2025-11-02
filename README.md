# Whatsapp Export MSG Parser

a little Rust Library to parse WhatsApp Message Exports into
an Vector of Messages

Message Struct
```rust
pub struct Message {
    pub date: String,
    pub datetime: String,
    pub sender: String,
    pub message: String,
}
```

Libary includes optional conversion to serde or JSON
