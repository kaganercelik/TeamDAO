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

<h4>Verify keypair</h4>

```bash
solana-keygen pubkey ~/my-solana-wallet/my-keypair.json
```

<h5>Output</h5>

```bash
ErRr1caKzK8L8nn4xmEWtimYRiTCAZXjBtVphuZ5vMKy
```

```bash
solana-keygen verify <PUBKEY> ~/my-solana-wallet/my-keypair.json
```

<h3>Anchor.toml</h3>

```
[provider]
cluster = "localnet"
wallet = "~/my-solana-wallet/my-keypair.jsonn"
```

### Building

```bash
yarn
npm install
```

```bash
anchor build
anchor keys list
```
  Take the output of program id. Copy and paste it into Anchor.toml ```toml team_dao = "DX9sn7m7pn3zQJP5B5oD5YQVQWxen9CX77u8rEqMFC41" </p>``` and ```rust declare_id!("DX9sn7m7pn3zQJP5B5oD5YQVQWxen9CX77u8rEqMFC41");``` here.

