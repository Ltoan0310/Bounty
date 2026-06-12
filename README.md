# Soroban Bounty Trust Protocol

## Project Title
Soroban Bounty Trust Protocol

## Project Description
Soroban Bounty Trust Protocol is a decentralized, secure, and automated smart contract platform designed to manage open challenges, hackathons, and bounties. Built using Soroban on the Stellar blockchain, the protocol allows creators to lock funds safely within a trustless escrow contract. Participants can then submit solutions, and upon providing the verified answer, the smart contract automatically resolves the bounty and transfers the locked funds instantly to the winner, removing any centralized intermediary risks.

## Project Vision
The vision of Soroban Bounty Trust Protocol is to foster global decentralized collaboration by providing a transparent, trustless escrow mechanism for developers and creators. By enforcing financial commitments on-chain, it builds absolute trust between bounty creators and participants, ensuring that correct work is always guaranteed immediate and automated payout.

## Key Features
- **Secure Fund Escrow:** Bounty creators lock token rewards directly inside the contract, ensuring proof of funds for participants.
- **Automated Payouts:** Immediate and trustless fund distribution to the participant who provides the correct cryptographic solution.
- **Decentralized Storage:** All bounty details, amounts, and statuses are permanently recorded on the Stellar ledger for public auditing.
- **Access & State Control:** Built-in safeguards to ensure bounties can only be claimed once (`is_resolved` status management).
- **Stellar Asset Compatibility:** Full integration with Stellar Native Assets (XLM) or any compliant CEP-41/SAC tokens.

## Usage Instructions
1. **Approve Token Allowance:** The creator approves the contract to pull the bounty reward amount from their wallet using the token contract's `approve` function.
2. **Create Bounty:** Call the `create_bounty` function with the creator's address, token address, reward amount, and challenge description. This locks the funds in escrow and generates a unique `Bounty ID`.
3. **Query Bounty Status:** Anyone can publicly query the state of any challenge by calling `get_bounty` with the respective `Bounty ID`.
4. **Submit & Claim Reward:** Participants solve the challenge and invoke `submit_and_claim` with their address, the `Bounty ID`, and the `solution` string. If verified, the contract dispenses the locked tokens instantly to the participant.

## Future Scope
- **Cryptographic Solution Hashing:** Implement SHA-256 hashing for solutions so correctness can be verified on-chain without revealing the answer in plaintext inside the transaction history.
- **Multi-voter Consensus / Oracle Judges:** Allow a decentralized panel of judges or oracles to vote on subjective or complex project submissions.
- **Milestone-based Payouts:** Support multi-stage bounties where funds are released incrementally based on progressive milestone completions.
- **Deadline & Expiry Clawbacks:** Add expiration timestamps allowing creators to safely reclaim locked funds if no correct solution is submitted within a specific timeframe.
- **Web3 Frontend Integration:** Build a responsive React/TypeScript user interface connected with Freighter Wallet for seamless user interaction.

## Technology Stack
- **Rust & Soroban SDK (v25):** For writing highly secure, predictable, and optimized WebAssembly (Wasm) smart contracts.
- **Stellar Blockchain:** For low-cost, lightning-fast decentralized ledger state management.
- **Stellar CLI:** For advanced pipeline building, network deployments, and contract invocations.

## Contribution
Community contributions are highly encouraged! Whether you are a Rust engineer, blockchain auditor, or UI/UX designer, feel free to fork this repository, open issues, and submit pull requests to enhance the protocol.

## License
This project is licensed under the MIT License.

---

### Contract Details
- **Network:** Stellar Testnet
- **Creator Account (Source):** `GAMHPVAWKRLML74ENMVRULCJXRDLE6MVYXHL3BGC67UDMEMAFDEBVSJM`
- **Supported Token (XLM Testnet):** `CDLZ6G2PYA36Y6SI7I667OBOZ56S6EO66SJZ7K3GCBH66N2IOK5HIAB6`
- **Active Contract ID:** `CB2BRUMMW75Y35LT7LK2CZDT7TI4JH2J66QEETFUAG5ZYQ7CDRXJSRCV`