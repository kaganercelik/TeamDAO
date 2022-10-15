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

Build again

```bash
anchor build
```

### Test

```bash
anchor test
```

## Features

<ul>
  <li>Create a team account</li>
  <li>Add a member to the team</li>
  <li>Remove a member from the team</li>
  <li>Transfer captainship of the team</li>
  <li>Leave team</li>
  <li>Init a tournament proposal</li>
  <li>Vote for the tournament proposal</li>
  <li>Leave the tournament</li>
  <li>Init distribution percentage proposal</li>
  <li>Distribution percentage proposal handler</li>
  <li>Can join tournament decider</li>
  <li>Claim reward</li>
</ul>


## Rules

<ul>
  <li>
    <h3>Create Team</h3>
    <p>
      <ul>
        <li>
          Sets the signer as the captain of the team and add the address as a member of the team.
        </li>    
    </ul>
    </p>
  </li>
  
   <li>
    <h3>Add member to the team</h3>
    <p>
      <ul>
        <li>
          A team can only have 5 players max.
        </li>
        <li>
          Cant add dublicate pubkey
        </li>
        <li>
          Only the captain of the team can add a member
        </li>
    </ul>
    </p>
  </li>
  
   <li>
    <h3>Remove a member from the team</h3>
    <p>
      <ul>
        <li>
          There must be more than 1 member in the team to remove a member
        </li>    
        <li>
          Only the captain of the team can remove a member
        </li>
        <li>
          There must be a member in the team with the given pubkey parameter.
        </li>
    </ul>
    </p>
  </li>
  
   <li>
    <h3>Transfer Captain</h3>
    <p>
      <ul>
        <li>
          Only the captain of the team can transfer captainship
        </li>    
         <li>
          There must be a member with the given pubkey parameter in the team
        </li>
    </ul>
    </p>
  </li>  

  <li>
    <h3>Leave Team</h3>
    <p>
      <ul>
        <li>
          If the captain wants to leave the team it transfer the captainship to the member after captain
        </li>    
        <li>
          If team has only 1 member in the team it resets the team account
        </li>
    </ul>
    </p>
  </li>

   <li>
    <h3>Init tournament</h3>
    <p>
      <ul>
        <li>
          Only the captain of the team can init a tournament proposal
        </li>   
         <li>
          If the team has already an active tournament proposal cant be started. In order to join another tournament the active tournament must be left first.
        </li> 
    </ul>
    </p>
  </li>
  
   <li>
    <h3>Vote For Tournament</h3>
    <p>
      <ul>
        <li>
          
        </li>    
    </ul>
    </p>
  </li>
  
   <li>
    <h3>Vote For Tournament</h3>
    <p>
      <ul>
        <li>
          
        </li>    
    </ul>
    </p>
  </li>
 
</ul>
