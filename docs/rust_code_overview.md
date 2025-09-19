# 🦀 Connexa Rust Code Overview

This document provides an overview and quick navigation for the Rust source code files within the `server/` directory of the Connexa project.

---

## Table of Contents
- [Core Server Files](#core-server-files)
- [API Endpoints](#api-endpoints)
- [Cryptography Modules](#cryptography-modules)
- [Other Files](#other-files)

---

## Core Server Files

*   [`main.rs`](../server/main.rs) - Main entry point of the server application.
*   [`relay.rs`](../server/relay.rs) - Handles message relaying logic.
*   [`signaling.rs`](../server/signaling.rs) - Manages signaling for real-time communication.
*   [`messaging.rs`](../server/messaging.rs) - Core messaging logic.
*   [`media.rs`](../server/media.rs) - Handles media-related operations.
*   [`webrtc.rs`](../server/webrtc.rs) - WebRTC related functionalities.
*   [`group.rs`](../server/group.rs) - Group management logic.
*   [`group_mls.rs`](../server/group_mls.rs) - MLS (Messaging Layer Security) implementation for groups.
*   [`device.rs`](../server/device.rs) - Device management logic.
*   [`auth.rs`](../server/auth.rs) - Authentication related logic.

---

## API Endpoints

*   [`api.rs`](../server/api.rs) - General API definitions and routing.
*   [`device_api.rs`](../server/device_api.rs) - API endpoints for device management.
*   [`group_api.rs`](../server/group_api.rs) - API endpoints for group management.
*   [`group_message_api.rs`](../server/group_message_api.rs) - API endpoints for group messages.
*   [`sfu_api.rs`](../server/sfu_api.rs) - API endpoints for SFU (Selective Forwarding Unit) interactions.

---

## Cryptography Modules

*   [`crypto/mod.rs`](../server/crypto/mod.rs) - Module for cryptographic operations.
*   [`crypto/double_ratchet.rs`](../server/crypto/double_ratchet.rs) - Double Ratchet algorithm implementation.
*   [`crypto/session.rs`](../server/crypto/session.rs) - Session management for cryptographic keys.
*   [`crypto/x3dh.rs`](../server/crypto/x3dh.rs) - X3DH (Extended Triple Diffie-Hellman) key exchange implementation.

---

## Other Files

*   [`build.rs`](../server/build.rs) - Build script for the Rust project.
