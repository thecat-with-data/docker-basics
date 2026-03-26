# **Third Project**

## Overview

A Rust web service in Docker to show how compiled languages are containerized and deployed with Compose.

## How to run

1. Install Docker Desktop.
2. Open terminal in `third-project`.
3. Start stack:

```bash
docker compose up --build
```

4. Open `http://localhost:3000`.
5. Clean up:

```bash
docker compose down
```

## What students should learn

- Rust build inside a container and the resulting artifact delivery.
- Why containerized builds ensure reproducible compilations.
- Compose networking with a custom network (`rustnetwork`).
- Exposing and mapping ports from container to host.
