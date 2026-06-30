# aruaru

**aruaru** is an open-source AI-native development platform for building, deploying, learning, and managing Web, Desktop, Mobile, and infrastructure applications with cost-aware AI routing and strict quality gates.

日本語では、aruaru は **Web・Desktop・Mobileアプリ開発、VPS運用、AI開発支援、学習、資格模擬試験、デプロイ自動化** を統合するオープンソース基盤です。

## Mission

- Make app and website development easier, safer, cheaper, and more enjoyable.
- Provide a second Cursor / second Claude / second ChatGPT / second Grok style development assistant as open-source software.
- Provide a second KUSANAGI style VPS and HTTPS automation platform for beginners and experts.
- Support low-spec PCs, old Windows machines, and users who cannot afford expensive GPU workstations.

## Product Structure

```text
aruaru
├─ aruaru-core          # Non-AI core plugins
├─ aruaru-ai-core       # AI-related core plugins and AI routing
├─ aruaru-web           # Web platform; includes aruaru-ai
├─ aruaru-desktop       # Desktop app with Windows installer target
├─ aruaru-mobile        # Mobile app target for Android / iPhone
├─ aruaru-db            # PostgreSQL + aruaru-db + distributed DB concepts
├─ aruaru-sftp          # SFTP / SSH / deployment automation
├─ aruaru-learning      # English, exams, data science, game-style learning
├─ aruaru-cert          # Mock exam certificates
└─ docs                 # Architecture and design documents
```

## Key Ideas

- **aruaru-core**: central non-AI plugin set.
- **aruaru-ai-core**: central AI plugin set and the AI control layer for aruaru-ai.
- **aruaru-web**: main Web product. aruaru-ai is included inside aruaru-web.
- **aruaru-sftp**: upload and deploy apps to rental VPS servers through SFTP / SSH.
- **Second KUSANAGI function**: domain, subdomain, HTTPS, certificate renewal, and health reporting automation.
- **AI model selection**: auto or manual selection among ChatGPT, Claude Opus, Grok, Gemini, DeepSeek, Qwen, local LLM, and aruaru-llm.
- **Cost-aware AI routing**: use cheaper AI first, then high-end AI only when needed.
- **Quality gate**: AI output is not trusted blindly. Build, test, security, and regression checks must pass.

## MVP Scope

MVP v0.1 focuses on:

- aruaru-web boot screen
- project registration
- AI manual selection
- simple AI auto-selection
- hardware diagnosis
- SFTP connection settings
- server directory creation
- simple file upload
- domain / subdomain registration design
- HTTPS monitoring design
- PostgreSQL connection design

## Low-Spec PC Policy

The project is intentionally designed for low-spec PCs. The founder is currently developing from a difficult financial situation and still uses a GT730-class PC. Therefore, aruaru must not assume an expensive GPU workstation. Heavy AI processing should be routed to external AI, cloud AI, server AI, or lightweight local models when appropriate.

## Development Stack

```text
Backend:       Rust + Poem
Frontend:      TypeScript + HTML5 + CSS3
Graph/API:     WunderGraph Cosmo + Versionless API
Database:      PostgreSQL + aruaru-db concept
Desktop:       Windows installer target, later macOS/Linux
Mobile:        Android / iPhone target
Deployment:    SFTP / SSH / VPS automation
```

## Documentation

See [`docs/README.md`](docs/README.md).

## Repository Status

This repository is currently in the design and MVP planning stage.
