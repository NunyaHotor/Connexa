# 🏗️ Connexa Architecture

This document describes the high-level architecture of Connexa, including cryptography, networking, storage, and client-server interactions. It serves as a reference for developers and contributors.  

---

## 1. Overview

Connexa is a **decentralized, end-to-end encrypted messaging platform**.  
It combines three layers:  
- **Clients** — Mobile, web, and desktop apps.  
- **Relays / Servers** — Federated or self-hosted message relays.  
- **Optional P2P** — Direct peer-to-peer connections for resilience.  

---

## 2. Cryptography

- **Key Exchange:** X3DH (Extended Triple Diffie-Hellman)  
- **Session Management:** Double Ratchet Algorithm (Signal protocol)  
- **Groups:** MLS (Messaging Layer Security) for scalable encrypted groups  
- **Metadata Protection:**  
  - Sealed sender (hide sender identity from relay)  
  - Private contact discovery (PSI/OPRF)  
  - Optional mixnets / cover traffic for anonymity  

---

## 3. Networking

- **Transport:** TLS + gRPC/WebSockets (client ↔ server)  
- **Relay Servers:** Store-and-forward messages until delivery  
- **P2P Layer (future):** libp2p for peer discovery and direct communication  
- **Media Delivery:** Encrypted upload/download (S3/IPFS backends)  

---

## 4. Storage

- **Client-side:**  
  - Encrypted SQLite (mobile/desktop)  
  - Local message history + key store  
- **Server-side:**  
  - Minimal metadata  
  - Encrypted message envelopes (deleted after delivery)  
  - Media blobs stored separately  

---

## 5. Clients

- **Mobile:** Native iOS (Swift) & Android (Kotlin/Java)  
- **Web:** React/Next.js frontend  
- **Desktop:** Electron wrapper or native (future)  
- **Features:**  
  - Encrypted chats, groups, calls  
  - Device linking & sync  
  - Configurable privacy controls  

---

## 6. Federation

- **Model:** Similar to Matrix, but with Connexa-specific protocol  
- **Homeservers:** Each user can host/join a trusted server  
- **Interoperability:** Federation protocol for message routing between servers  
- **Optional P2P:** Fallback to peer-to-peer if federation nodes are unavailable  

---

## 7. Security Considerations

- Minimize server trust (servers can only see encrypted envelopes)  
- Forward secrecy & post-compromise security  
- Auditability: open-source crypto libraries & independent audits  
- Anti-abuse: spam protection, rate-limiting, moderation tools  

---

## 8. Future Extensions

- Decentralized identity (DID / ENS integration)  
- Marketplace for relays, storage, and services  
- End-to-end encrypted channels & bots  
- Formal verification of crypto components  
