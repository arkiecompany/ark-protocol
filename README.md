# ark-protocol

**Open protocol and reference implementation** for presenting **many local ports on a VPS as one standardized HTTPS/WebSocket front** toward Cloudflare (or any TLS client). It is **not tied to Ark**: any control plane, script, or operator can adopt the [manifest](docs/manifest.md) and HTTP routes described here.

**New here?** Read **[Using the manifest](docs/using-the-manifest.md)** — it walks through authoring JSON, validation, Worker bindings, Ark resolved env, and the Rust / JS packages in one place.

This repository is laid out **by name under `ark-protocol`**:

| Path | Role |
|------|------|
| [`spec/mux-manifest.v1.schema.json`](spec/mux-manifest.v1.schema.json) | Version **1** JSON Schema (file name keeps `mux-manifest` for stable `$id` URLs). |
| [`crates/ark-protocol/`](crates/ark-protocol) | **Rust crate `ark-protocol`** — import as `ark_protocol`; **`manifest_json_for_deploy`** builds the Worker `MUX_MANIFEST_JSON` string from resolved env. |
| [`packages/ark-protocol/`](packages/ark-protocol) | **npm `@tschk/ark-protocol`** (ESM + `.d.ts`) — **`manifestJsonForDeploy`** for **Bun** / **Node 18+**. |
| [`docs/`](docs/) | Architecture, VPS, Cloudflare, manifest fields, worker bindings. |
| [`worker/entry.mjs`](worker/entry.mjs) | Reference **Cloudflare Worker** (`/protocol/v1/…`, legacy `/__ark/…`). |

## Why

On a VPS you often run several processes (HTTP API, WebRTC signaling, game server, metrics). Cloudflare and browsers want **one or few public origins**. This project standardizes:

1. A **machine-readable manifest** of “logical name → upstream host:port” (and optional TLS/SNI hints).
2. A **single ingress** on the VPS (reverse proxy, multiplexer, or tunnel) that fans traffic out using that manifest.
3. A **documented HTTP adapter surface** (`/protocol/v1/adapter/…`) for bridging WebSocket or TCP through HTTPS at the edge.

## Quick links

- [Using the manifest](docs/using-the-manifest.md) — **start here**: end-to-end usage (VPS, Worker, Ark, libraries).
- [Overview](docs/overview.md) — goals and vocabulary.
- [VPS deployment](docs/vps.md) — multi-port → single listener on your server.
- [Cloudflare](docs/cloudflare.md) — tunnels, Workers, and how they attach to that single port.
- [Manifest v1](docs/manifest.md) — schema walkthrough and validation.
- [Worker routes](docs/worker.md) — paths and Worker `plain_text` bindings used by the reference Worker.

## Reference Worker (Wrangler)

```bash
cd "$(dirname "$0")"
npx wrangler@3 dev
```

Bindings are documented in [`docs/worker.md`](docs/worker.md). Ark and other systems may upload the same `worker.js` via the Cloudflare API with different binding values.

## License

Mozilla Public License 2.0 — see [`LICENSE`](LICENSE).
