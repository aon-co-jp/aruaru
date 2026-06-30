# Plugin Architecture

aruaru uses a plugin-based architecture so features can be added, disabled, replaced, or isolated.

## Goals

- Reduce bug impact.
- Allow optional installation.
- Share functionality between Web, Desktop, and Mobile.
- Keep AI and non-AI responsibilities separated.

## Plugin Rules

Every plugin must have:

- `plugin_id`
- `name`
- `version`
- `type`
- `permissions`
- `dependencies`
- `enabled / disabled` state
- failure isolation

## Example Manifest

```toml
[plugin]
id = "aruaru-ai-core"
name = "Aruaru AI Core"
version = "0.1.0"
type = "ai-core"
enabled = true

[permissions]
network = true
filesystem = true
database = true
ai_api = true
ssh = false

[dependencies]
required = ["aruaru-core"]
optional = ["aruaru-db", "aruaru-sftp"]
```

## Core Boundary

- `aruaru-core` contains central plugins that do not require AI.
- `aruaru-ai-core` contains central plugins that require AI or model routing.
