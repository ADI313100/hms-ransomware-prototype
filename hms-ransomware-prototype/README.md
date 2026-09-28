
# HMS Ransomware Prototype

## Prerequisites
- Docker Desktop
- Rust (`rustup`)
- Python 3.11+
- Node.js 20+

## Start infrastructure
```bash
docker compose up -d
```

## Backend API
```bash
cd backend-api
cargo run
```
Runs at http://127.0.0.1:8080

## React Dashboard
```bash
cd dashboard
npm install
npm run dev
```
Open http://localhost:5173

## AI Analytics
```bash
cd ai-analytics
pip install -r requirements.txt
python main.py
```

## Simulator
```bash
cd simulator
pip install -r requirements.txt
python app.py
```

## Rust Agent
```bash
cd agent
cargo run
```
