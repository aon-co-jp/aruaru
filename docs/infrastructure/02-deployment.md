# Deployment Architecture

aruaru deployment should be safe and reversible.

## Flow

1. Check server connection.
2. Upload public key when needed.
3. Create app directory.
4. Upload files.
5. Set permissions.
6. Run health check.
7. Check HTTPS.
8. Create rollback point.

## MVP

MVP may start with simple file upload and directory creation before full zero-downtime deployment.
