# 📦 Klutter

**Klutter**는 일상에서 유용하게 사용할 수 있는 소소하고 "잡다구리"한 웹 도구들을 모아놓은 오픈소스 프로젝트입니다. Rust(Axum)와 SvelteKit 스택으로 개발되었습니다.

🔗 **Live Demo:** [https://klutter.vercel.app/](https://klutter.vercel.app/)

---

## 🛠 Tech Stack

### Backend
- **Language:** Rust
- **Framework:** [Axum](https://github.com/tokio-rs/axum)
- **Utilities:** Tower-HTTP, Tracing, Serde, Anyhow

### Frontend
- **Runtime:** [Deno](https://deno.com/)
- **Language:** TypeScript
- **Framework:** [SvelteKit](https://kit.svelte.dev/) (Svelte 5 Runes)
- **Styling:** [Tailwind CSS](https://tailwindcss.com/)

---

## 🚀 Deployment Guide

이 프로젝트는 백엔드와 프론트엔드가 분리된 구조로 배포됩니다.

### 1. Backend (Render.com)
- **Runtime:** Rust
- **Root Directory:** `backend`
- **Build Command:** `cargo build --release`
- **Start Command:** `cargo run --release`
- **Environment Variables:** `PORT=3000` (Render에서 자동 할당)

### 2. Frontend (Vercel)
- **Framework Preset:** SvelteKit (Deno)
- **Root Directory:** `frontend`
- **Build Command:** `deno install && deno task build`
- **Environment Variables:**
    - `PUBLIC_API_URL`: 배포된 Render 백엔드 주소


---

## 🌿 Git Branch Strategy

본 프로젝트는 안정적인 개발과 배포 관리를 위해 다음과 같은 브랜치 전략을 사용합니다.

### 브랜치 구조
- **`deploy`**: 운영 환경에 배포되는 최상위 브랜치입니다. 모든 안정화된 코드는 최종적으로 이곳에 모입니다.
- **`master`**: 개발의 중심이 되는 브랜치입니다. 모든 기능 개발과 버그 수정이 완료된 코드가 가장 먼저 합쳐지는 곳입니다.
- **`feature/*` / `fix/*`**: 새로운 기능을 추가하거나 버그를 수정하기 위한 단기 브랜치입니다.

### 작업 흐름 (Workflow)
1. 새로운 작업(기능 추가, 수정)은 `master` 브랜치에서 별도의 브랜치(`feature/*` 등)를 생성하여 진행합니다.
2. 작업 및 검증이 완료되면 해당 브랜치를 `master` 브랜치로 머지(Merge)합니다.
3. 배포 준비가 완료된 시점에 `master` 브랜치의 내용을 `deploy` 브랜치로 머지하여 운영 환경에 최종 반영합니다.

---

## 💻 Local Development

### Prerequisites
- [Rust & Cargo](https://rustup.rs/)
- [Deno](https://deno.com/)

### Setup
1. Repository 클론
2. 백엔드 실행: `cd backend && cargo run`
3. 프론트엔드 설정: `cd frontend && deno install`
4. 프론트엔드 실행: `deno task dev --open`

---

## ✍️ Included Tools
- **Luxury**: 영문 텍스트를 멋진 필기체 유니코드로 변환
- *More tools coming soon...*
