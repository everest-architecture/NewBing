<p align="center">
  <img width="180" src="./public/logo.png" alt="NewBing">
  <h1 align="center">NewBing</h1>
  <p align="center">NewBing Desktop Application (Windows Mac  Linux)</p>
</p>


![License](https://img.shields.io/badge/License-Apache%202-green.svg)
![visitor](https://visitor-badge.glitch.me/badge?page_id=everest-architecture.NewBing)
[![NewBing downloads](https://img.shields.io/github/downloads/everest-architecture/NewBing/total.svg?style=flat-square)](https://github.com/everest-architecture/NewBing/releases)
[![chat](https://img.shields.io/badge/chat-discord-blue?style=flat&logo=discord)](https://discord.gg/TEhtK5S6x2)


## Install
### Windows
- [NewBing_0.0.1_x64_en-US.msi](https://github.com/everest-architecture/NewBing/releases/download/v0.0.1/NewBing_0.0.1_x64_en-US.msi)

### Mac
- [NewBing_0.0.1_x64.dmg](https://github.com/everest-architecture/NewBing/releases/download/v0.0.1/NewBing_0.0.1_x64.dmg)

### Usage
### Recommended
- [Rust (Required)](https://www.rust-lang.org/)
- [Node.js (Required)](https://nodejs.org/)
- [VS Code](https://code.visualstudio.com/)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Dev
```bash
git clone https://github.com/everest-architecture/NewBing.git

cd NewBing

pnpm install

pnpm dev

pnpm build

pnpm tauri build
```

## FAQ
### Downloading wix311-binaries.zip
- When executing the `pnpm tauri build` command, it may get stuck at `Downloading https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip`. In this case, you can:
  - Manually download `https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip`.
  - Create the `C:\Users\user\AppData\Local\tauri\WixTools` directory
  - Copy the files extracted from `wix311-binaries.zip` to `C:\Users\user\AppData\Local\tauri\WixTools`


## Thanks
- This project is inspired by [ChatGPT](https://github.com/lencx/ChatGPT)

## License

Apache License
