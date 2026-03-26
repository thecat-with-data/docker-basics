# **Second Project**

## Overview

A Python Flask app in Docker demonstrating code-with-volume, environment variable use, and local development.

## How to run

1. Install Docker Desktop.
2. Open terminal in `second-project`.
3. Start services:

```bash
docker compose up --build
```

4. Browse `http://localhost:5000`.
5. Stop when finished:

```bash
docker compose down
```

## What students should learn

- Building and running Python apps in containers.
- The benefit of bind mount `.:/app` for live code changes.
- Flask dev server and `FLASK_ENV=development` in containerized workflows.
- How Compose service configuration maps ports and handles restarts.
