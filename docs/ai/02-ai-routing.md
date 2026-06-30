# AI Routing

aruaru-ai-core must choose the right AI for the task.

## Task Examples

```text
Rust:                    ChatGPT / Claude / Opus priority
Poem:                    official docs + ChatGPT / Grok / Gemini comparison
WunderGraph Cosmo:       official docs + Grok / Gemini / ChatGPT comparison
Complex debugging:       Opus / high-reasoning ChatGPT / multi-AI compare
Repeated bug loops:      root-cause analysis + regression tests + quality gate
Low-spec PC:             external AI first, aruaru-llm as assistant
```

## Manual UI

```text
□ Use aruaru-llm together
□ Save cost
□ Prefer quality
□ Prefer bug fixing
□ Require official docs check
□ Compare multiple AI models
```

## Important Rule

AI output must not be applied directly to production. It must pass quality gates first.
