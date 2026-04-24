# Project Title

Job Listing – A Soroban Smart Contract dApp on Stellar

## Project Vision

This project demonstrates a **job posting platform with stake deposits** built on Stellar Testnet using Soroban smart contracts. It provides:
- How to write a Soroban smart contract in Rust
- How to manage persistent storage for job postings and applications
- How to handle user authentication and stake management in smart contracts
- How to deploy and interact with contracts on Stellar Testnet

The goal is to provide a working example that employers can use to post jobs with stake deposits and seekers can apply.

---

## Description

A Soroban smart contract dApp that allows employers to post jobs with a stake deposit on Stellar Testnet. When an employer posts a job, they deposit a stake which is returned when the position is filled with an accepted applicant.

---

## Features

### 1. Job Posting with Stake Deposit
- Employers post jobs with a stake deposit
- Job postings include title, salary, and deposit amount
- Deposit is held until job is filled or cancelled

### 2. Application System
- Job seekers can apply to open positions
- Applications tracked on-chain for transparency
- Employer reviews and accepts applicants

### 3. Stake Management
- Employer deposit is returned when applicant is accepted
- Deposit can serve as signing bonus for selected candidate
- Full transparency on stake status

### 4. On-chain Transparency
- All job postings stored permanently on blockchain
- Application status verifiable by anyone
- Complete audit trail for stake movements

---

## Contract Functions

- **post_job(job_id, employer, title, salary, deposit)** – Employer posts job with stake deposit
- **apply(job_id, seeker)** – Job seeker applies to a position
- **accept_applicant(job_id, seeker)** – Employer accepts applicant, returns deposit
- **get_job(job_id)** – Returns job details (employer, title, salary, deposit, filled)
- **get_applicants(job_id)** – Returns list of applicant addresses

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CC76MXXPPLYDMYCIRQNI2EW5IKNVIGJDSYP67QDDNYR4VYSRNY6SOTIT](https://stellar.expert/explorer/testnet/tx/b9461407f85085a01c6fd76e3607536d9268964fb73e956a204c86629d4420f0)

![screenshot](https://i.ibb.co/n8qHBY0J/image.png)

---

## Future Scopes

### 1. Escrow with Native Token
- Integrate with Stellar's native XLM for stake deposits
- Automatic refund on job cancellation

### 2. Commission Model
- Platform takes a percentage of the stake as fee
- Automatic commission distribution

### 3. Reputation System
- Add ratings for employers and job seekers
- On-chain reputation tracking

### 4. Multiple Applications Limit
- Limit number of active applications per seeker
- Priority queue for oversubscribed positions

### 5. Time-based Features
- Job expiration dates
- Auto-fill after set period

### 6. Frontend dApp
- Build a web interface for easier interaction
- Dashboard for employers and seekers

---

## Technical Stack

- **Language**: Rust
- **Framework**: Soroban SDK
- **Network**: Stellar Testnet

---

## Profile

- **Name:** kyhnkotzurdu872