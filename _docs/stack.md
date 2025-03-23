# Tech Stack Overview
This project is a web-based tool for creating time-coded notes on music tracks and playing back the referenced sections using Spotify. It is built with a modern, performance-oriented stack that emphasizes speed, flexibility, and future scalability.

## Project Goals
- Take timestamped notes on music
- Integrate with Spotify for interactive audio playback
- Structure codebase for possible multiplayer or collaborative features

## Stack Breakdown
### Frontend: React + TypeScript + Vite (with Bun)
- React
- TypeScript: Adds type safety to catch errors early and align well with the strongly-typed Rust backend.
- Vite
- Bun: Used in place of Node for better dev performance.

### Backend: Rust + Axum
- Rust
- Axum: A modern async web framework built on Tokio, designed for type safety and composability.

Chosen to explore Rust in a real-world app and gain performance and safety advantages over dynamic languages.

### Database: Kuzu (Graph DB)
Kùzu is an embeddable graph database optimized for complex relationships and lightweight deployments.

Chosen specifically to evaluate its suitability for production use at work, and to model annotation relationships (e.g., note ↔ song ↔ user).

Graph structure is a natural fit for features like linked notes, collaborative history, and track metadata relationships.

### Audio Playback: Spotify Web Playback SDK (Frontend)
Enables high-quality audio streaming and playback control directly in the browser.

Supports seeking to specific timecodes and integrates with a user's Spotify account.

Chosen for fast prototyping — no need to host or upload audio files. Requires Spotify Premium.

## Monorepo Structure
Everything is organized under a single Git repo, with clearly separated folders for frontend and backend:

```
/frontend   → React app managed by Bun
/backend    → Axum server with Rust
.env        → Shared config values (e.g. Spotify credentials)
```

## Summary
This stack was chosen to balance:

- Rapid solo development
- Long-term maintainability
- High performance
- A good learning opportunity (Rust + Kùzu)

The app starts as a personal tool, but is structured with potential for growth into a collaborative, multi-user platform.
