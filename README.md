<div align="center">
  <h1>👻 GhostHandDesk</h1>
  <p><strong>Application de bureau Windows pour contrôle à distance sécurisé via relay VPS — prenez la main sur n'importe quelle machine.</strong></p>

  ![Version](https://img.shields.io/badge/version-0.5.1-blue)
  ![Stack](https://img.shields.io/badge/stack-Tauri%20v2%20%2B%20Rust%20%2B%20Vue3-purple)
  ![License](https://img.shields.io/badge/license-MIT-green)
  ![Platform](https://img.shields.io/badge/platform-Windows-0078D6)
</div>

---

## 📋 Description

GhostHandDesk est une application de bureau Tauri v2 permettant le contrôle à distance de machines Windows via un relay VPS centralisé. Le relay gère la mise en relation entre le client et le poste cible sans exposition directe des ports — seul le relay VPS est accessible publiquement. Conçue pour être légère, sécurisée et simple d'utilisation.

---

## 🎬 Démonstration

<video src="https://media.heiphaistos.org/videos/ghosthanddesk.mp4" controls width="100%" preload="none"></video>

---

## ✨ Fonctionnalités

- **Connexion relay VPS** : Mise en relation sécurisée via serveur relay — aucun port à ouvrir sur la machine cible
- **Contrôle à distance** : Prise en main complète du bureau distant (souris, clavier, affichage)
- **Gestion des sessions** : Historique des connexions, reconnexion automatique, timeout configurable
- **Rejet auto-connexion** : Protection contre les connexions non sollicitées (réponse 400 explicite)
- **Interface moderne** : UI Vue 3 intégrée dans Tauri, fluide et réactive
- **Installeur NSIS** : Distribution Windows avec installeur silencieux
- **Relay PM2** : Serveur relay déployé sur VPS avec gestion de processus PM2

---

## 🛠️ Stack technique

| Couche | Technologies |
|--------|-------------|
| Application desktop | Tauri v2 · Rust |
| Interface utilisateur | Vue 3 · TypeScript · Vite · TailwindCSS |
| Relay serveur | Node.js · Express · WebSocket |
| Déploiement relay | VPS · PM2 · nginx |
| Distribution | Installeur NSIS Windows |

---

## 🚀 Installation

### Installation bureau (Windows)

Télécharger l'installeur depuis les [Releases GitHub](https://github.com/Heiphaistos/GhostHandDesk/releases/tag/v0.5.1) :

```
GhostHandDesk_0.5.1_x64-setup.exe
```

Lancer l'installeur et suivre les étapes. L'application est disponible dans le menu Démarrer après installation.

### Build depuis les sources

#### Prérequis

- Node.js >= 20
- Rust >= 1.77 (stable)
- Tauri CLI v2 (`cargo install tauri-cli`)
- WebView2 Runtime (inclus sur Windows 11)

#### Développement

```bash
# Cloner le dépôt
git clone https://github.com/Heiphaistos/GhostHandDesk.git
cd GhostHandDesk

# Installer les dépendances frontend
npm install

# Lancer en mode développement
npm run tauri dev
```

#### Build production

```bash
# Build installeur NSIS
npm run tauri build

# L'installeur se trouve dans :
# src-tauri/target/release/bundle/nsis/GhostHandDesk_0.5.1_x64-setup.exe
```

---

## 🖥️ Déploiement du Relay VPS

Le relay est déployé sur le VPS et géré par PM2.

### Configuration

```bash
# Sur le VPS, déployer le relay
cd /opt/ghosthanddesk
npm install

# Lancer avec PM2 (--cwd requis)
pm2 start server.js --name ghosthanddesk --cwd /opt/ghosthanddesk

# Sauvegarder la config PM2
pm2 save
```

### Variables d'environnement relay

```env
PORT=3025
RELAY_SECRET=votre_secret_relay
MAX_SESSIONS=50
SESSION_TIMEOUT_MS=300000
```

### Configuration nginx (relay)

```nginx
server {
    listen 443 ssl;
    server_name relay.heiphaistos.org;

    location / {
        proxy_pass http://127.0.0.1:3025;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

---

## 📁 Structure du projet

```
GhostHandDesk/
├── src/               # Frontend Vue 3
│   ├── components/    # SessionPanel, ConnectionStatus, Controls
│   ├── stores/        # Pinia (session, config)
│   └── views/         # Main, Settings
├── src-tauri/         # Backend Rust Tauri
│   ├── src/
│   │   ├── relay.rs   # Client relay WebSocket
│   │   ├── capture.rs # Capture écran
│   │   └── input.rs   # Injection souris/clavier
│   └── Cargo.toml
├── server/            # Relay Node.js (VPS)
│   └── server.js
└── deploy/            # Scripts déploiement VPS
```

---

## 📸 Aperçu

![screenshot](./docs/screenshot.png)

---

## 🔐 Sécurité

- Relay VPS : seul point d'entrée, les machines cibles ne sont pas exposées directement
- Rejet automatique des tentatives de connexion non autorisées (HTTP 400)
- Sessions avec timeout configurable
- Authentification par secret partagé entre client et relay
- Logs des connexions IP + timestamp

---

## 📝 Licence

MIT — © 2026 Heiphaistos
