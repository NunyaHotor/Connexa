# 🔐 Connexa Protocol

Connexa uses a **hybrid cryptographic protocol** combining proven ideas from the Signal Protocol and Messaging Layer Security (MLS).  
This document explains how messages are structured (`proto/message.proto`) and how encryption works at different stages.  

---

## 1. Identity & Keys

Each user has:  
- **Identity key pair (Ed25519/X25519)** — long-term identity.  
- **Signed pre-key** — refreshed periodically, signed by identity key.  
- **One-time pre-keys** — ephemeral keys published for initial sessions.  
- **Device keys** — each device has its own key pair, linked to the account.  

Keys are distributed via:  
- A **directory service** (for bootstrapping), or  
- **Peer exchange** in decentralized mode.  

---

## 2. Session Establishment (1:1)

Connexa uses **X3DH (Extended Triple Diffie-Hellman)** for initial key exchange.  
Steps:  
1. Nketsi fetches Mawutor’s pre-keys.  
2. Nketsi performs X3DH to establish a shared secret.  
3. Nketsi sends her first encrypted message (in a `MessageEnvelope`) to Mawutor.  
4. Mawutor derives the same session keys and decrypts.  

After this, both parties switch to the **Double Ratchet Algorithm** for forward secrecy and post-compromise security.  

---

## 3. Message Structure

All messages follow the schema in [`proto/message.proto`](../proto/message.proto).  

- **MessageEnvelope**:  
  - Metadata wrapper (ID, sender, recipient, timestamp).  
  - Contains an encrypted `MessagePayload`.  

- **MessagePayload**:  
  - Decrypted content: `TextMessage`, `MediaMessage`, `CallMessage`, or `ControlMessage`.  
  - Control messages handle **group events** and **device linking**.  

**Example Flow (1:1 Text):**  
Nketsi writes "Hello Mawutor".
↓
TextMessage → MessagePayload → serialized with Protobuf
↓
Encrypt payload with Double Ratchet session key
↓
Wrap in MessageEnvelope
↓
Send to relay → Mawutor
↓
Mawutor decrypts → sees TextMessage
---

## 4. Groups (MLS)

For groups, Connexa uses **Messaging Layer Security (MLS)**:  
- Efficient group key management (logarithmic complexity).  
- Each group has a **GroupState** with a shared secret.  
- Adding/removing members updates the group key tree.  

Group events are carried as **GroupControl** messages inside `ControlMessage`.  

---

## 5. Media

Media files (images, videos, documents) are:  
1. Encrypted with a random symmetric key.  
2. Uploaded to a storage backend (S3, IPFS, or relay).  
3. A **MediaMessage** containing `media_url` and `media_key` is sent.  
4. Recipient downloads file, decrypts with the key.  

---

## 6. Calls (WebRTC)

- Calls are set up using **WebRTC**.  
- Call signaling (SDP offers/answers, ICE candidates) is carried inside a `CallMessage`.  
- Media streams are encrypted end-to-end (DTLS-SRTP).  
- Group calls use an **SFU (Selective Forwarding Unit)** — but streams remain E2EE.  

---

## 7. Multi-Device

- Each user may link multiple devices.  
- A new device performs X3DH with existing devices to sync session keys.  
- Device events are represented as **DeviceControl** messages.  
- Messages are fan-out encrypted: sender encrypts once per recipient device.  

---

## 8. Metadata Protection

Connexa minimizes metadata leakage:  
- **Sealed Sender:** Relay cannot see who sent a message.  
- **Minimal Logs:** Servers delete undelivered messages after a TTL.  
- **Contact Discovery:** Uses Private Set Intersection (PSI/OPRF).  
- **Optional Cover Traffic:** Servers may inject dummy messages for anonymity.  

---

## 9. Federation & P2P

Two modes of operation:  
- **Federated Relays:** Similar to Matrix — servers forward encrypted envelopes.  
- **P2P Mode:** libp2p for direct client-to-client routing.  

In both cases, encryption guarantees that intermediaries cannot read messages.  

---

## 10. Security Guarantees

- **Confidentiality:** Only intended recipients can decrypt.  
- **Integrity:** Messages cannot be tampered with undetected.  
- **Forward Secrecy:** Past messages remain safe even if keys are compromised.  
- **Post-Compromise Security:** Recovery is possible after compromise.  
- **Deniability:** No cryptographic proof of who said what (like Signal).  

---

## 11. Future Extensions

- Mixnet integration for stronger anonymity.  
- Decentralized identity (DID, ENS).  
- Formal verification of protocol components.  
- Interoperability with Matrix and other secure messengers.  
## 📈 Example Sequence: Nketsi → Mawutor (1:1 Text Message)

```mermaid
sequenceDiagram
    participant N as Nketsi (Sender)
    participant R as Relay Server
    participant M as Mawutor (Recipient)

    N->>R: Fetch Mawutor's pre-keys (X3DH)
    N->>M: Establish session with X3DH
    Note right of N: Double Ratchet session established

    N->>R: Send MessageEnvelope (encrypted payload)
    R->>M: Deliver MessageEnvelope
    M->>M: Decrypt with Double Ratchet<br/>→ sees TextMessage
