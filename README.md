# solana-final-project
Full Stack dApp (will be multichain in the future)

Modules to download

For MacOS/Linux 

sh -c "$(curl -sSfL https://release.solana.com/v1.14.11/install)"

For Windows

curl https://release.solana.com/v1.14.11/solana-install-init-x86_64-pc-windows-msvc.exe --output C:\solana-install-tmp\solana-install-init.exe --create-dirs

C:\solana-install-tmp\solana-install-init.exe v1.14.11

Checking whether solana is installed

solana --version

Installing Anchor

Prerequisites: Rust, Yarn and Solana

Anchor installation

cargo install --git https://github.com/project-serum/anchor avm --locked --force

Checking anchor 

anchor --version

Creating Anchor project

anchor init new-workspace-name

