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

## API 方針: REST 全廃・gRPC / Connect へ (2026-11-04)

### 日本語
- **REST は全廃する。新しい API は REST で作らない。** ブラウザや `curl` から呼ぶ API は **Connect の単純呼び出し**(`POST /<パッケージ>.<Service>/<Method>`、`content-type: application/json`、本文は JSON)、サービス間は **gRPC / gRPC-web**(RPoem の管理API `open_runo.admin.v1.Admin` と同じ流儀)。ストリーミングは、必要になってから。
- **既存の REST は、触るときに順に移す。** 移したら旧 REST の経路は消す。移行が終わるまで残るが、**新規には追加しない**。
- **JSON は RS-JSON に統一**する(読み込みは `from_str_secure` / `parse_secure` でサイズ・深さ・重複キーの上限つき、書き出しは `rust_json::fast`)。ブラウザの WASM に `prost` は入れない(サイズが増えるため)。
- **例外(自分たちの API ではなく、相手の規格が決めているもの)**: ACME、OAuth/OIDC のコールバック、第三者(GitHub など)からの Webhook、ヘルスチェック、静的ファイル配信、OTLP など。これらは規格どおりの HTTP を受ける。
- 参考実装: `RPoem`(`open_runo.admin.v1.Admin`)、`open-easy-web`(`server/src/connect.rs`、`proto/open_easy_web/admin/v1/admin.proto`)。
- ※ このリポジトリのコードが既に方針どおりかどうかは、各リポジトリの現状による。この節は方針の記録であり、コードの変更を意味しない。

### English
- **REST is being abolished. New APIs are not built as REST.** APIs called from browsers or `curl` use **Connect unary calls** (`POST /<package>.<Service>/<Method>`, `content-type: application/json`, JSON body); service-to-service calls use **gRPC / gRPC-web** (the same style as RPoem's admin API `open_runo.admin.v1.Admin`). Streaming only when it is actually needed.
- **Existing REST endpoints are migrated one by one, when they are touched.** The old REST route is removed once migrated. Until migration is finished they remain, but **no new ones are added**.
- **JSON is unified on RS-JSON**: reading with `from_str_secure` / `parse_secure` (size, depth and duplicate-key limits), writing with `rust_json::fast`. `prost` is not put into browser WASM (it would grow the binary).
- **Exceptions (shapes dictated by someone else's standard, not our own API)**: ACME, OAuth/OIDC callbacks, webhooks from third parties (e.g. GitHub), health checks, static file serving, OTLP, and so on. These accept plain HTTP as the standard requires.
- Reference implementations: `RPoem` (`open_runo.admin.v1.Admin`) and `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Note: whether the code in a given repository already follows this policy depends on that repository. This section records the policy; it does not mean the code has been changed.

### Deutsch (KI-Übersetzung, noch nicht von Muttersprachlern geprüft)
- **REST wird abgeschafft. Neue APIs werden nicht als REST gebaut.** APIs, die aus dem Browser oder mit `curl` aufgerufen werden, nutzen **Connect-Unary-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`, JSON-Body); zwischen Diensten gilt **gRPC / gRPC-web** (wie bei der Admin-API von RPoem, `open_runo.admin.v1.Admin`). Streaming erst, wenn es wirklich nötig ist.
- **Bestehende REST-Endpunkte werden einzeln migriert, sobald man sie anfasst.** Nach der Migration wird die alte REST-Route entfernt. Bis dahin bleiben sie bestehen, es werden aber **keine neuen hinzugefügt**.
- **JSON wird auf RS-JSON vereinheitlicht**: Lesen mit `from_str_secure` / `parse_secure` (Grenzen für Größe, Tiefe und doppelte Schlüssel), Schreiben mit `rust_json::fast`. `prost` kommt nicht in Browser-WASM (die Datei würde größer).
- **Ausnahmen (Formate, die ein fremder Standard vorgibt, nicht unsere eigene API)**: ACME, OAuth/OIDC-Callbacks, Webhooks von Dritten (z. B. GitHub), Health-Checks, statische Dateien, OTLP usw. Diese nehmen schlichtes HTTP entgegen, wie es der Standard verlangt.
- Referenzimplementierungen: `RPoem` (`open_runo.admin.v1.Admin`) und `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Hinweis: Ob der Code eines Repositorys diese Richtlinie schon befolgt, hängt vom jeweiligen Repository ab. Dieser Abschnitt hält die Richtlinie fest; er bedeutet nicht, dass der Code bereits geändert wurde.

### Русский (машинный перевод, носителями языка ещё не проверен)
- **REST полностью отменяется. Новые API не создаются как REST.** API, вызываемые из браузера или через `curl`, используют **унарные вызовы Connect** (`POST /<пакет>.<Service>/<Method>`, `content-type: application/json`, тело — JSON); между сервисами — **gRPC / gRPC-web** (так же, как административный API RPoem, `open_runo.admin.v1.Admin`). Потоковая передача — только когда она действительно нужна.
- **Существующие REST-эндпоинты переносятся по одному, когда до них доходят руки.** После переноса старый REST-маршрут удаляется. До завершения переноса они остаются, но **новые не добавляются**.
- **JSON унифицируется на RS-JSON**: чтение через `from_str_secure` / `parse_secure` (ограничения на размер, глубину и повторяющиеся ключи), запись через `rust_json::fast`. `prost` не добавляется в браузерный WASM (размер вырос бы).
- **Исключения (форматы, заданные чужим стандартом, а не наш собственный API)**: ACME, обратные вызовы OAuth/OIDC, вебхуки сторонних сервисов (например, GitHub), проверки работоспособности, раздача статических файлов, OTLP и т. п. Они принимают обычный HTTP, как требует стандарт.
- Эталонные реализации: `RPoem` (`open_runo.admin.v1.Admin`) и `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Примечание: соответствует ли код конкретного репозитория этой политике, зависит от самого репозитория. Этот раздел фиксирует политику и не означает, что код уже изменён.
