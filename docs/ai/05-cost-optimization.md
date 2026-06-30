# Cost Optimization

The goal is maximum quality with minimum cost.

## Strategy

```text
Cheap AI first
↓
If failed or risky
↓
Mid-tier AI
↓
If still failed or high complexity
↓
High-end AI such as Opus / high-reasoning models
```

## Opus Policy

Opus should be selectable, but not used for every task by default.

Recommended uses:

- complex bug analysis
- repeated bug loops
- difficult architecture design
- long specification review
- high-stakes refactoring

Simple tasks should use cheaper AI or local models when possible.

## GPU Purchase Warning

aruaru should explain that buying many expensive professional GPUs may be unrealistic for ordinary users. In many cases, using paid AI services only when necessary is cheaper and safer than purchasing a large local GPU cluster.
