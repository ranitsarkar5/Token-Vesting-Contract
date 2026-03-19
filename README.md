# 🚀 Token Vesting Contract (Soroban - Stellar)

## 📌 Project Description
This project implements a Token Vesting Smart Contract using Soroban on the Stellar blockchain. It allows tokens to be released gradually over time to a beneficiary instead of all at once.

This is useful for:
- Team token allocations
- Investor lockups
- Reward distribution systems

---

## ⚙️ What it does
The contract locks a fixed amount of tokens and releases them linearly over a specified duration.

Key workflow:
1. Contract is initialized with:
   - Beneficiary address
   - Total token amount
   - Vesting start time
   - Vesting duration
2. Tokens become available gradually over time.
3. Beneficiary can claim (release) vested tokens anytime.

---

## ✨ Features
- 🔐 Secure token locking mechanism  
- ⏳ Time-based linear vesting  
- 📈 Real-time vested amount calculation  
- 💸 Partial token release support  
- 🔍 Public state query function  
- ⚡ Lightweight and efficient Soroban contract  

---

## 🧠 How Vesting Works
- Before start time → No tokens released  
- During vesting → Tokens released proportionally  
- After duration → Full amount available  

---

## 🛠 Tech Stack
- Rust (Soroban SDK)
- Stellar Blockchain (Soroban Smart Contracts)

---

## 🔗 Deployed Smart Contract Link
Token Vesting Contract:  
https://lab.stellar.org/smart-contracts/contract-explorer?$=network$id=testnet&label=Testnet&horizonUrl=https:////horizon-testnet.stellar.org&rpcUrl=https:////soroban-testnet.stellar.org&passphrase=Test%20SDF%20Network%20/;%20September%202015;&smartContracts$explorer$contractId=CARYWO4GSPJJSC6DJQHR6JYHPTZSJTCOKV63ZDKHH4ENNS7GMWFNUJBE;;

Example:
https://stellar.expert/explorer/testnet/tx/f3721f1f0274210527166482d07d3c88eb944917a5f71b299760b8762299bcee

---

## 📦 Future Improvements
- Add cliff period support  
- Add multiple beneficiaries  
- Integrate with token contract for automatic transfers  
- Add admin controls  

---

## 👨‍💻 Author
Ranit Sarkar
---

## 📄 License
MIT License
