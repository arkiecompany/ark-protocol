# Using the mux manifest (v1)

This page explains **what the manifest is for** and **how people actually use it** in practice.

## What it is

The **mux manifest v1** is a small JSON document (`version`, `id`, `listen`, `routes`) that describes how **incoming traffic** should map to **upstream host:port** (and a protocol hint). Same file is used in three places:

| Where | What you do |
|-------|-------------|
| **On a VPS** | Your reverse proxy, tunnel agent, or custom `muxd` reads the JSON (or config generated from it) and listens on **`listen`**, then forwards using **`routes`**. See [vps.md](vps.md). |
| **On Cloudflare Workers** | Put the JSON string into the Worker binding **`MUX_MANIFEST_JSON`**. The reference [`worker/entry.mjs`](../worker/entry.mjs) parses it on every request and proxies when a route matches. See [worker.md](worker.md). |
| **With Ark** | Put either a full **`MUX_MANIFEST_JSON`** object (or string) or a **`MUX_DEFAULT_UPSTREAM`** URL in the service’s **resolved environment** JSON. On deploy, Ark fills the Worker’s `MUX_MANIFEST_JSON` binding for you (via the Rust / JS helpers below). |

The normative shape is defined in [`spec/mux-manifest.v1.schema.json`](../spec/mux-manifest.v1.schema.json). A filled example lives at [`examples/manifest.v1.example.json`](../examples/manifest.v1.example.json).

## 1. Author the JSON

Edit a file (for example `manifest.json`) so it validates against the schema. At minimum you need `version: "1"`, an `id`, a `listen` block, and at least one entry in `routes` with `name`, `match`, and `upstream`.

## 2. Validate (optional but recommended)

```bash
# from the ark-protocol repo root, with Python + jsonschema installed
python3 -c "
import json, jsonschema
from pathlib import Path
root = Path('.')
schema = json.loads((root / 'spec/mux-manifest.v1.schema.json').read_text())
data = json.loads((root / 'examples/manifest.v1.example.json').read_text())
jsonschema.Draft202012Validator(schema).validate(data)
print('ok')
"
```

## 3. Use it on the edge (Cloudflare Worker)

- **Manual / Wrangler:** set a `plain_text` binding **`MUX_MANIFEST_JSON`** to the **string** contents of your file (escape quotes as needed), or use `[vars]` in `wrangler.toml` for small fixtures.
- **Programmatic:** serialize the same object to a string and send it in the Workers upload `metadata.bindings` array next to `worker.js` (same shape Ark uses).

On the Worker, **`listen` is ignored** today; only **`routes`** drive behaviour at the edge. Upstream hosts must be reachable from Cloudflare (not `127.0.0.1`).

## 4. Use it without writing JSON by hand

If you only have a single public HTTPS origin and want the default “everything under `/` goes there” route:

- **Resolved env (Ark or your own JSON):** set **`MUX_DEFAULT_UPSTREAM`** to a full URL, e.g. `https://api.example.com:8443`.
- **Or** embed a full manifest under **`MUX_MANIFEST_JSON`** (object or JSON string). That wins over `MUX_DEFAULT_UPSTREAM` when both are set.

Helpers that emit the same string Ark puts on the Worker:

| Stack | Package / crate | Path in repo | API |
|-------|-----------------|--------------|-----|
| **Rust** | `ark-protocol` | [`crates/ark-protocol`](../crates/ark-protocol) | `manifest_json_for_deploy(deployment_id, service_id, &resolved_env)` |
| **Bun / Node** | `@tschk/ark-protocol` | [`packages/ark-protocol`](../packages/ark-protocol) | `manifestJsonForDeploy(deploymentId, serviceId, resolvedEnv)` |

**From another repo (path):**

```toml
ark-protocol = { git = "https://github.com/tschk/ark-protocol", branch = "main", package = "ark-protocol" }
```

```json
"@tschk/ark-protocol": "file:../ark-protocol/packages/ark-protocol"
```

## 5. Use it only on the server

You do **not** have to use the Worker. Point **`cloudflared`**, nginx, Caddy, or HAProxy at the same JSON (or a config file generated from it). The Worker and the VPS multiplexer are **alternatives** that share one schema.

## See also

- [manifest.md](manifest.md) — field-by-field reference.
- [overview.md](overview.md) — goals and vocabulary.
- [cloudflare.md](cloudflare.md) — tunnels vs Workers vs DNS.
