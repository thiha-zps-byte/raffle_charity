# Raffle Charity

A Soroban smart contract dApp for charity event fundraising on the Stellar blockchain.

## Project Vision

Raffle Charity is a decentralized application built on the Stellar network using the Soroban smart contract platform. It demonstrates:
- How to write a Soroban smart contract in Rust
- How to manage persistent storage (raffle state, ticket ownership)
- How to handle role-based actions (admin vs. participant) in smart contracts
- How to deploy and interact with contracts on Stellar Testnet

The goal is to provide a transparent, on-chain raffle system where proceeds go directly to a designated charity address.

---

## Description

A Soroban smart contract that enables an admin to create a charity raffle on Stellar Testnet. Participants purchase tickets, and when the admin draws a winner, the prize is awarded to the winner while the proceeds are transferred to the designated charity address. All raffle state is stored permanently on-chain, ensuring full transparency.

---

## Features

### 1. Admin-Created Raffles
- Admin initializes and creates raffles with a defined prize, ticket price, and charity recipient
- Each raffle has a unique ID for reference

### 2. Ticket Purchasing
- Users buy tickets by calling `buy_ticket` with their address
- Each ticket is tracked on-chain with its buyer's address and a sequential ticket index
- Ticket purchases are atomic — the buyer's address is stored at the current sold-count index

### 3. Transparent On-chain Drawing
- The admin draws the winner after ticket sales close
- The winner is selected pseudorandomly from the sold tickets
- Raffle state (completed flag, winner address) is permanently recorded on-chain

### 4. Charity Payout
- Proceeds (sold_count * ticket_price) are intended to go to the charity address upon drawing
- Winner receives the prize amount; the charity receives the ticket sale proceeds

### 5. Raffle State Query
- Anyone can query the current state of any raffle by its ID
- Returns charity address, ticket price, prize, sold count, completion status, and winner (if drawn)

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CDXNSUXPNHKH35AJWTREEFXQ4BEL2FCIHMWVKNG5AZTVNKO77FNVMEIJ](https://stellar.expert/explorer/testnet/tx/97650e842ea278dc8b4e2b4b6c2cfa2385c24954b8e7615cbfa323a6fa91b6c3)

![screenshot](https://i.ibb.co/SwJjRvxQ/image.png)

---

## Contract Interface

| Method | Description |
|--------|-------------|
| `init` | Initialize the contract |
| `create_raffle(raffle_id, charity_address, ticket_price, prize)` | Admin creates a raffle |
| `buy_ticket(raffle_id, buyer)` | User purchases a ticket |
| `draw_winner(raffle_id)` | Admin draws and announces the winner |
| `get_raffle(raffle_id)` | Returns full raffle state |

---

## Future Scopes

### 1. VRF-based Randomness
- Integrate a verifiable random function (VRF) for trustless, unpredictable winner selection

### 2. Multiple Active Raffles
- Allow multiple simultaneous raffles with independent state tracking

### 3. Tokenized Tickets
- Mint NFTs representing raffle tickets for enhanced traceability

### 4. Frontend dApp
- Build a React or plain HTML/JS web interface for easier user interaction

### 5. Automatic Payouts
- Integrate Stellar's payment channels to automate prize and proceeds distribution upon drawing

### 6. Timed Raffles
- Add start/end timestamps so raffles auto-close and trigger drawing automatically

### 7. Governance
- Allow token holders to vote on charity recipients or fee parameters

---

## Profile

- **Name:** :thiha-zps-byte
