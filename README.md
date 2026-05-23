# Stellar Chat DApp

**Stellar Chat DApp** - Blockchain-Based Decentralized Chat Application

---

## Project Description

Stellar Chat DApp is a decentralized messaging application built on the Stellar blockchain using Soroban SDK and Rust programming language.

This smart contract allows users to send, retrieve, and delete chat messages directly on-chain without relying on centralized servers or traditional databases.

All chat data is stored securely inside the smart contract instance storage, ensuring transparency, persistence, and decentralization through blockchain technology.

Each message contains:
- Unique message ID
- Username
- Message content

The application demonstrates how decentralized communication systems can be implemented using Stellar Soroban Smart Contracts.

---

## Project Vision

Our vision is to create a decentralized communication platform where users fully control their conversations and data ownership without depending on centralized messaging services.

### Main Goals

- **Decentralized Communication**  
  Eliminate centralized chat servers and move messaging systems to blockchain infrastructure.

- **User Ownership**  
  Ensure users fully own and manage their chat data.

- **Transparency & Security**  
  Guarantee message integrity using blockchain verification.

- **Immutable Messaging**  
  Prevent unauthorized modification of chat records.

- **Trustless Platform**  
  Build a communication system secured by smart contracts instead of third-party companies.

---

# Key Features

## 1. Send Messages

- Users can send chat messages directly to blockchain storage
- Each message contains username and message content
- Automatic unique ID generation
- Fast and secure transactions on Stellar network

---

## 2. Retrieve Messages

- Fetch all stored chat messages
- Easy integration with frontend applications
- Real-time blockchain data synchronization

---

## 3. Delete Messages

- Remove messages using message ID
- Automatically update on-chain storage
- Efficient message management system

---

## 4. Blockchain Transparency

- All chat activities stored transparently on blockchain
- Verifiable smart contract interactions
- Immutable transaction records

---

## 5. Stellar Network Integration

- Built on Stellar blockchain
- Powered by Soroban Smart Contracts
- Low-cost transaction fees
- Fast execution performance

---

# Smart Contract Structure

```rust
pub struct ChatMessage {
    id: u64,
    username: String,
    message: String,
}
```

---

# Contract Functions

## send_message()

Send a new chat message.

### Parameters:
- `username` → sender username
- `message` → chat content

---

## get_messages()

Retrieve all stored chat messages.

---

## delete_message()

Delete a chat message using its ID.

### Parameters:
- `id` → unique message ID

---

# Example Chat Data

```rust
ChatMessage {
    id: 1001,
    username: "Asep",
    message: "Hello Stellar Blockchain!"
}
```

---

# Contract Details

- Contract Address:
'CDKI5YWDBXIAO6BWS52IXISB2A7FR4RRBVP27BEFSDB2R2FDIK4KXL2J'

---

# Future Scope

## Short-Term Features

### 1. Message Timestamp
Store message sending time using ledger timestamp.

### 2. User Authentication
Integrate wallet authentication for user identity verification.

### 3. Chat Rooms
Support public and private chat rooms.

### 4. Message Search
Search messages by username or keyword.

---

## Medium-Term Features

### 5. Private Messaging
Enable wallet-to-wallet communication.

### 6. Realtime Notifications
Push notifications for incoming messages.

### 7. Media Sharing
Support image and file attachments using IPFS.

### 8. Message Editing
Allow users to edit their own messages.

---

## Long-Term Vision

### 9. End-to-End Encryption
Secure messages with encryption technology.

### 10. Decentralized Frontend Hosting
Deploy frontend using IPFS or decentralized hosting.

### 11. DAO Governance
Community-driven development and governance.

### 12. Cross-Chain Messaging
Enable messaging between multiple blockchain networks.

---

# Technical Requirements

- Rust Programming Language
- Soroban SDK
- Stellar Blockchain Network

---

# Getting Started

Deploy the smart contract to Stellar Soroban network and interact using these functions:

- `send_message()` → Send chat message
- `get_messages()` → Retrieve all messages
- `delete_message()` → Delete message by ID

---

# Technologies Used

- Rust
- Soroban SDK
- Stellar Blockchain

---

# Conclusion

Stellar Chat DApp demonstrates how blockchain technology can be used to build decentralized communication systems with transparency, security, and user ownership.

This project serves as a foundation for future decentralized social and messaging applications on Stellar Network.

---

**Stellar Chat DApp** — Decentralized Messaging on Stellar Blockchain 🚀