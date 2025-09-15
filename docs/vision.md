# Vision, Goals & Features

## Vision
Connexa is an open, decentralized messaging platform designed to combine the **privacy of Signal**, the **flexibility of Telegram**, and the **ubiquity of WhatsApp**.  
Our vision is to empower people everywhere with a secure, censorship-resistant, and user-friendly communication tool that they can **trust, self-host, and extend** — without sacrificing usability.  

Connexa is built on three principles:  
1. **End-to-end security by default** — every message, call, and media file is protected with strong encryption.  
2. **Decentralization and user sovereignty** — users can choose to run their own servers, rely on trusted relays, or connect peer-to-peer.  
3. **Modern, familiar user experience** — all the features people expect from today’s messaging apps, without the compromises on privacy.  

---

## Goals
- **Security**: Implement audited, modern cryptography (X3DH, Double Ratchet, MLS).  
- **Privacy**: Minimize metadata exposure with sealed sender, contact discovery via private set intersection, and optional anonymity layers.  
- **Resilience**: Federated and peer-to-peer architecture for censorship resistance.  
- **Usability**: Match or exceed the usability of mainstream messengers (media sharing, group chat, calls).  
- **Extensibility**: Open APIs for bots, channels, integrations, and developer contributions.  
- **Transparency**: Fully open-source, open protocol, with third-party audits and community governance.  

---

## Features (Planned)

### Core Messaging
- ✅ End-to-end encrypted 1:1 chats  
- ✅ Encrypted group chats (scalable via MLS)  
- ✅ Disappearing & deletable messages  
- ✅ Typing indicators, read receipts (optional)  

### Media & Sync
- 📎 Encrypted file, photo, and video sharing  
- 💬 Message history synced across devices  
- 🔍 Local search & message management  

### Calls
- 📞 End-to-end encrypted 1:1 voice & video calls (WebRTC)  
- 👥 Group calls with SFU (scalable forwarding units)  

### Decentralization
- 🌍 Hybrid architecture: federated relays + optional P2P messaging  
- ⚡ Self-hostable homeservers (Rust/Go backend)  
- 🔑 Metadata protection: sealed sender & private contact discovery  

### User Experience
- 📱 Native iOS & Android clients  
- 💻 Desktop and Web clients  
- 👤 Multiple account models (phone number, username, DID)  
- 🎨 Stickers, reactions, rich presence  

### Advanced
- 🔒 Optional anonymity mode (mixnets / cover traffic)  
- 🤖 Bots, channels, and public groups  
- 🛡️ Anti-spam, moderation, and abuse-resistant design  
