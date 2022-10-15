# TeamDAO
Team handler DAO on Solana  Blockchain using Anchor Framework

## Requirements

  <ul>
    <li>Rust installation -> <a href="https://www.rust-lang.org/tools/install">here</a></li>
    <li>Solana installation -> <a href="https://docs.solana.com/cli/install-solana-cli-tools">here</a></li>
    <li>Yarn installation -> <a href="https://yarnpkg.com/getting-started/install">here</a></li>
    <li>Anchor installation -> <a href="https://www.anchor-lang.com/docs/installation">here</a>
    <li>Git installation -> <a href="https://git-scm.com/book/en/v2/Getting-Started-Installing-Git">here</a>
  </ul>


## Getting Started

### Cloning project

```bash
git clone https://github.com/fuujazz/TeamDAO.git
code TeamDao
```
### Creating local wallet

```bash
mkdir ~/my-solana-wallet
solana-keygen new --outfile ~/my-solana-wallet/my-keypair.json
```

<h3>Anchor.toml</h3>

```
 [provider]
cluster = "localnet"
wallet = "~/my-solana-wallet/my-keypair.jsonn"
```
