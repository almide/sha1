# almide/sha1

SHA-1 hash for Almide. Hardware-accelerated on Rust target via AES-NI, pure Almide fallback for WASM.

## Usage

```almide
import sha1
import bytes

let digest = sha1.hash(bytes.from_string("hello"))
let hex = sha1.hex(digest)  // "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"
```

## API

| Function | Type | Description |
|---|---|---|
| `hash(data)` | `Bytes -> Bytes` | SHA-1 digest (20 bytes) |
| `hex(digest)` | `Bytes -> String` | Hex-encode a digest |

## Performance

| Target | Implementation | SHA-1 x10000 |
|---|---|---|
| Rust | Native (sha1 crate, AES-NI) | ~1ms |
| WASM | Pure Almide | ~860ms |

## Install

```bash
almide add almide/sha1
```
