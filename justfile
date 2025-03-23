# justfile

default: dev

dev:
    # Run both frontend and backend in parallel
    just -f justfile start-backend & just -f justfile start-frontend

start-backend:
    cd backend && cargo run

start-frontend:
    cd frontend && bun dev

kill:
    pkill -f "cargo run" || true
    pkill -f "bun dev" || true
