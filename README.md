# Cliently Mac

**Cliently** is a modern, sovereign business management and client portal platform designed for freelancers, agencies, and independent professionals. 

This repository contains the **macOS native desktop application** for Cliently, built using [Tauri](https://tauri.app/).

## 🚀 Overview

Cliently bridges the gap between client communication, invoicing, project management, and automated workflows. The desktop application provides a native, highly responsive interface to the Cliently ecosystem, empowering you to manage your business directly from your macOS dock without being tethered to a browser tab.

**Key capabilities of the Cliently ecosystem include:**
- **Secure Deliverable Vaulting:** Securely seal and deliver final assets to clients.
- **Integrated AI Assistance (Neo):** A built-in "AI CTO" to help automate your workflows, draft responses, and analyze business metrics.
- **Client Management & Invoicing:** Streamlined billing and project tracking.
- **Native Desktop Performance:** Leverages Tauri and Rust for a lightweight footprint and fast execution compared to traditional Electron apps.

*(Note: The core backend services, web dashboard, and proprietary AI daemon are maintained in separate, private repositories to ensure data security and protect core intellectual property.)*

## 🛠️ Tech Stack

- **Framework:** [Tauri](https://tauri.app/)
- **Frontend:** React, TypeScript, Next.js
- **Styling:** Tailwind CSS, shadcn/ui
- **Systems Language:** Rust

## 📦 Getting Started

To build and run the Cliently Mac app locally, you will need to set up the Tauri prerequisites for macOS.

### Prerequisites

1. **Node.js** (v18 or higher)
2. **Rust** (latest stable)
3. **Xcode Command Line Tools** (`xcode-select --install`)

For a detailed setup guide on macOS, refer to the [Tauri Prerequisites Documentation](https://tauri.app/v1/guides/getting-started/prerequisites#macos).

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/albin435/Cliently-Mac.git
   cd Cliently-Mac
   ```

2. Install dependencies:
   ```bash
   npm install
   # or
   yarn install
   # or
   pnpm install
   ```

3. Set up environment variables:
   Create a `.env` file in the root directory and configure your endpoints and public keys (e.g., Supabase URL, Anon Key). *Never commit your `.env` file.*

4. Run the application in development mode:
   ```bash
   npm run tauri dev
   ```

5. Build for production:
   ```bash
   npm run tauri build
   ```
   This will generate a `.dmg` and `.app` file in the `src-tauri/target/release/bundle/macos/` directory.

## 🔒 Security

Security is a primary focus for Cliently. 
- All sensitive API keys and secrets are strictly excluded from this repository and must be provided via local `.env` files.
- The app utilizes strict Content Security Policies (CSP) defined in `tauri.conf.json`.
- Row-Level Security (RLS) is enforced at the database level for all user data.

## 🤝 Contributing

While this is an open-source mirror of the Mac application intended to showcase the architecture and UI/UX, we welcome feedback, bug reports, and pull requests! 

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.
