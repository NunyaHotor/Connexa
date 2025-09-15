# 🛣️ Connexa Roadmap

Connexa is being developed in **phases**, starting with a secure core and expanding toward full decentralization, rich features, and large-scale usability.  

---

## Phase A — Foundation 🔑  
**Goal:** Establish the core security model, repo structure, and development environment.  

- [ ] Define protocol spec (X3DH + Double Ratchet for 1:1, MLS for groups)  
- [ ] Document threat model and architecture (`docs/architecture.md`)  
- [ ] Create message schema (`proto/message.proto`)  
- [ ] Minimal relay server (Rust/Go) with health check + message store-and-forward  
- [ ] CI/CD setup (GitHub Actions: build + test)  
- [ ] Basic README + documentation (`docs/vision.md`, `docs/roadmap.md`)  

---

## Phase B — Core Messaging 💬  
**Goal:** Deliver a working, privacy-preserving MVP for encrypted 1:1 messaging.  

- [ ] Implement X3DH key exchange + Double Ratchet sessions  
- [ ] 1:1 encrypted text messages (with sealed sender to hide metadata)  
- [ ] Encrypted media sharing (client-side encryption → upload → encrypted download)  
- [ ] Store-and-forward delivery with push notification triggers  
- [ ] Native mobile clients (basic UI: send/receive text + media)  
- [ ] Local message persistence (SQLite on mobile)  

---

## Phase C — Groups & Multi-Device 👥  
**Goal:** Support secure groups and multi-device sync.  

- [ ] Implement MLS for efficient encrypted group messaging  
- [ ] Group management (create, invite, leave, remove)  
- [ ] Device linking flow (QR / verification)  
- [ ] Message sync across devices  
- [ ] Disappearing messages + local search  

---

## Phase D — Calls & Media 📞  
**Goal:** Enable real-time communication and richer media experience.  

- [ ] WebRTC 1:1 voice & video calls (end-to-end encrypted)  
- [ ] TURN server integration for NAT traversal  
- [ ] Group calls via SFU (e.g., Jitsi, Janus, Mediasoup)  
- [ ] Encrypted file & media previews, streaming playback  

---

## Phase E — Decentralization & Federation 🌍  
**Goal:** Expand architecture beyond centralized relays.  

- [ ] Federated homeservers (Matrix-style interoperability)  
- [ ] libp2p integration for peer discovery and optional P2P routing  
- [ ] IPFS/OrbitDB for optional decentralized media storage  
- [ ] Private contact discovery (PSI/OPRF protocol)  
- [ ] Metadata protection upgrades (sealed sender, batching, cover traffic)  

---

## Phase F — Advanced Features & Hardening 🛡️  
**Goal:** Make Connexa production-ready and extensible.  

- [ ] Channels, bots, and public groups  
- [ ] Stickers, reactions, rich presence  
- [ ] Admin & moderation tools for groups/channels  
- [ ] Anti-spam measures (rate limits, CAPTCHAs, proof-of-work)  
- [ ] Third-party security audits  
- [ ] Bug bounty program  
- [ ] Release 1.0 🎉  

---

## Long-Term Vision 🚀  
- Optional anonymity layer (mixnets / Tor integration)  
- Formal verification of cryptographic components  
- Decentralized governance of protocol specs  
- Marketplace for community-run relays and federation nodes  
