# Security Architecture

Security must be included from the beginning.

## Rules

- do not store passwords in plain text
- do not log secrets
- do not send API keys or private keys to AI models
- separate permissions by plugin
- require explicit user approval before deployment
- prefer least privilege
- validate remote paths before upload
- create backups before destructive operations

## AI Safety Rule

AI may propose changes, but aruaru must verify and apply them through controlled workflows.
