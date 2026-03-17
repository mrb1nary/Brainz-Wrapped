# 🎧 BrainzWrapped

Turn your ListenBrainz history into simple, visual insights.

---

## 🌐 Website

Coming soon: 

---

## ✨ Features

* Top artists and tracks
* Listening sessions and streaks
* Hourly and weekday patterns
* Yearly heatmap
* Listening age (which decade you listen to most)

---

## 🛠️ Tech Stack

**Backend**

* Rust (Axum, Tokio)
* ListenBrainz + MusicBrainz + Wikidata

**Frontend**

* Next.js (App Router)
* Tailwind CSS

---

## 🚀 Running Locally

### Backend

```bash id="xk9d2m"
cargo run
```

Runs on `http://localhost:3001`

### Frontend

```bash id="l2v7qf"
npm install
npm run dev
```

Runs on `http://localhost:3000`

---

## 🔌 API

```http id="p4n8zs"
GET /health
# Check if server is running

GET /stats/:username
# Full aggregated stats (artists, tracks, sessions, heatmap, etc.)

GET /top-artists/:username
# Most listened artists with play counts and images

GET /top-tracks/:username
# Most played tracks

GET /sessions/:username
# Listening session stats (total, average, longest)

GET /streaks/:username
# Current and longest listening streaks

GET /busiest-day/:username
# Day with highest number of listens

GET /hourly/:username
# Listening distribution across hours (0–23)

GET /weekday/:username
# Listening distribution across days of the week

GET /heatmap/:username
# Daily listen counts for the entire year (used for heatmap)

GET /listening-age/:username
# Distribution of listens by decade (e.g. 2000s, 2010s)
```

---

## 🔒 Notes

* No login required
* No data is stored
* Everything is computed on demand

---

Made with ❤️ in India
