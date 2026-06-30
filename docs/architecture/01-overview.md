# Architecture Overview

aruaru is an AI-native open-source development and operation platform.

## Goals

- Build Web, Desktop, and Mobile apps with AI assistance.
- Deploy apps and websites to VPS rental servers through SFTP and SSH.
- Automate domain, subdomain, HTTPS, certificate renewal, and health reporting.
- Provide learning, mock exams, and certificate generation.
- Support low-spec PCs and reduce dependency on expensive local GPU environments.

## Top-Level Modules

```text
aruaru-core       Non-AI center plugins
aruaru-ai-core    AI center plugins and AI routing
aruaru-web        Main Web product; includes aruaru-ai
aruaru-desktop    Desktop app and Windows installer target
aruaru-mobile     Android / iPhone target
aruaru-db         PostgreSQL, native DB, distributed DB, history
aruaru-sftp       SFTP / SSH / VPS deployment automation
aruaru-learning   Learning and exam platform
aruaru-cert       Mock certificate generation
```

## Design Principle

aruaru must separate production-ready MVP features from research features.

- MVP features must be simple, testable, and safe.
- Research features must be clearly marked as research until official information and benchmark data verify them.
