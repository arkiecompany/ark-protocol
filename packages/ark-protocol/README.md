# @tschk/ark-protocol

JavaScript / TypeScript (ESM + `.d.ts`) entrypoint for the **ark-protocol** repo: **`manifestJsonForDeploy`** builds the same **`MUX_MANIFEST_JSON`** string as the Rust crate **`ark-protocol`** (`crates/ark-protocol`) and Ark’s control plane.

```js
import { manifestJsonForDeploy } from "@tschk/ark-protocol";

const json = manifestJsonForDeploy("deploy-1", "svc-a", {
  MUX_DEFAULT_UPSTREAM: "https://origin.example.com",
});
```

See **[Using the manifest](../docs/using-the-manifest.md)** and the [repository README](../README.md).
