# **Initial Project**

## Overview

A simple static website served from a containerized web server. The goal is to introduce Dockerfile basics and Docker Compose orchestration for a minimal web app.

## How to run

1. Install Docker Desktop for Windows (or Docker Engine for Linux/macOS).
2. Open a terminal inside `initial-project`.
3. Start the stack:

```bash
docker compose up --build
```

4. Open `http://localhost:8080` in your browser.
5. When done, stop and clean up:

```bash
docker compose down
```

## What students should learn

- How a simple Dockerfile defines a filesystem, port, and startup command.
- How Docker Compose maps host ports (`8080`) to container ports (`80`).
- Why containers isolate apps from local environment dependencies.
- How to iterate quickly by rebuilding and restarting containers.
