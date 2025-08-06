# 🚀 TechNerds - Solana Staking Platform

<div align="center">

![TechNerds Logo](https://img.shields.io/badge/TechNerds-TN-black?style=for-the-badge&logo=solana&logoColor=white)

**A sophisticated, professional Solana staking platform built with modern web technologies**

[![Solana](https://img.shields.io/badge/Solana-Devnet-9945FF?style=flat-square&logo=solana&logoColor=white)](https://explorer.solana.com/address/6ywTdo75yF3Ssur2W5BLyPrVSujcQyA5FXpZsCNU4Kzs?cluster=devnet)
[![Next.js](https://img.shields.io/badge/Next.js-15.2.2-000000?style=flat-square&logo=next.js&logoColor=white)](https://nextjs.org/)
[![Anchor](https://img.shields.io/badge/Anchor-0.30.1-663399?style=flat-square&logo=rust&logoColor=white)](https://www.anchor-lang.com/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-3178C6?style=flat-square&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

</div>

## ✨ Features

### 🎨 **Modern UI/UX**
- **Sophisticated Black Theme** - Premium, professional design aesthetic
- **Glass Morphism Effects** - Modern blur and transparency effects
- **Responsive Design** - Perfect on desktop, tablet, and mobile
- **Smooth Animations** - Subtle hover effects and transitions

### 🔒 **Secure Staking**
- **Solana Integration** - Built on Solana blockchain for speed and efficiency
- **Multi-Duration Support** - Stake for 7, 14, 30, or 90 days
- **Real-time Portfolio** - Track your staking rewards and progress
- **Wallet Integration** - Support for popular Solana wallets

### ⚡ **Performance**
- **Next.js 15** - Latest React framework with Turbopack
- **Anchor Framework** - Secure smart contract development
- **TypeScript** - Type-safe development
- **Optimized Build** - Fast loading and smooth performance

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        TechNerds Platform                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────┐    ┌─────────────────┐    ┌──────────────┐ │
│  │   Frontend UI   │    │  Smart Contract │    │   Solana     │ │
│  │   (Next.js)     │◄──►│    (Anchor)     │◄──►│  Blockchain  │ │
│  │                 │    │                 │    │   (Devnet)   │ │
│  └─────────────────┘    └─────────────────┘    └──────────────┘ │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### Prerequisites

- **Node.js** 18.0+ 
- **Rust** 1.79.0 (required for Anchor)
- **Solana CLI** latest
- **Anchor CLI** 0.30.1

### Installation

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd if.1
   ```

2. **Install dependencies**
   ```bash
   # Frontend dependencies
   cd ui
   npm install
   
   # Smart contract dependencies  
   cd ../contract
   npm install
   ```

3. **Configure Solana**
   ```bash
   # Set to devnet
   solana config set --url devnet
   
   # Generate a new keypair (if needed)
   solana-keygen new
   
   # Airdrop some SOL for testing
   solana airdrop 2
   ```

### 🏃‍♂️ Running the Application

#### Frontend Development Server
```bash
cd ui
npm run dev
```
Visit [http://localhost:3000](http://localhost:3000)

#### Smart Contract Development
```bash
cd contract

# Build the program
anchor build

# Deploy to devnet
anchor deploy
```

## 📁 Project Structure

```
if.1/
├── ui/                          # Next.js Frontend
│   ├── app/                     # App router pages
│   ├── components/              # React components
│   │   ├── Body.tsx            # Main staking interface
│   │   ├── Header.tsx          # Navigation header
│   │   ├── AdminBody.tsx       # Admin dashboard
│   │   └── provider/           # Blockchain providers
│   ├── anchor-idl/             # Generated IDL files
│   └── public/                 # Static assets
│
├── contract/                   # Anchor Smart Contract
│   ├── programs/staking/       # Rust program source
│   │   ├── src/
│   │   │   ├── lib.rs         # Program entry point
│   │   │   ├── instructions/   # Program instructions
│   │   │   └── state.rs       # Account structures
│   │   └── Cargo.toml         # Rust dependencies
│   └── Anchor.toml            # Anchor configuration
│
└── README.md                  # This file
```

## 🔗 Smart Contract Details

**Program ID**: `6ywTdo75yF3Ssur2W5BLyPrVSujcQyA5FXpZsCNU4Kzs`

**Network**: Solana Devnet

**Key Instructions**:
- `initialize` - Initialize staking pool
- `deposit` - Stake SOL tokens
- `withdraw` - Unstake and claim rewards
- `claim_rewards` - Claim accumulated rewards

## 🎯 Usage Guide

### For Stakers

1. **Connect Wallet** - Click "Select Wallet" and choose your preferred Solana wallet
2. **Enter Amount** - Input the SOL amount you want to stake
3. **Choose Duration** - Select staking period (7, 14, 30, or 90 days)
4. **Stake SOL** - Confirm the transaction in your wallet
5. **Track Progress** - Monitor your staking rewards in the Portfolio section

### For Administrators

1. **Admin Panel** - Access via `/admin` route
2. **Manage Stakes** - View and manage all active stakes
3. **Configure Rewards** - Adjust APY rates and staking parameters

## 🛠️ Technology Stack

| Category | Technology | Version |
|----------|------------|---------|
| **Frontend** | Next.js | 15.2.2 |
| **Language** | TypeScript | 5.0+ |
| **Styling** | CSS Modules | - |
| **Blockchain** | Solana | Latest |
| **Smart Contracts** | Anchor | 0.30.1 |
| **Wallet** | Solana Wallet Adapter | Latest |

## 🎨 Design Philosophy

### **Sophisticated Black Theme**
- Elegant black gradients with subtle white accents
- Professional aesthetic suitable for financial applications
- Glass morphism effects for modern appeal

### **User Experience**
- Intuitive staking interface
- Real-time feedback and loading states
- Responsive design for all devices
- Smooth animations and transitions

## 🚦 Development

### Available Scripts

**Frontend (`/ui`)**
```bash
npm run dev        # Start development server
npm run build      # Build for production
npm run start      # Start production server
npm run lint       # Run ESLint
```

**Smart Contract (`/contract`)**
```bash
anchor build       # Build the program
anchor deploy      # Deploy to configured cluster  
anchor test        # Run tests
anchor upgrade     # Upgrade deployed program
```

## 🔐 Security

- **Anchor Framework** - Secure smart contract development
- **Type Safety** - Full TypeScript implementation
- **Wallet Integration** - Secure transaction signing
- **Devnet Testing** - Safe testing environment

## 🌟 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Support

If you have any questions or need support:

- 📧 **Email**: support@technerds.dev
- 💬 **Discord**: [TechNerds Community](https://discord.gg/technerds)
- 🐦 **Twitter**: [@TechNerdsSOL](https://twitter.com/TechNerdsSOL)

## 🙏 Acknowledgments

- **Solana Foundation** - For the amazing blockchain infrastructure
- **Anchor Team** - For the excellent smart contract framework
- **Next.js Team** - For the powerful React framework
- **Community** - For testing and feedback

---

<div align="center">

**Built with ❤️ by TechNerds**

*Empowering the future of decentralized finance*

</div>
