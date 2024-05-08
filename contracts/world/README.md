# Contracts 

```sh
brew install sui

sui move test


## Optional
sui client faucet

# Deploy
sui client publish --gas-budget 50000000

# Create a game
sui client call --package 0x5f234782d0d7fcb5412aafca6e87be1e6b2c67383566f2f4c499bc12ddafb385 --module Game --function create_game --args 0x1d658a40cdb1c3f2e814c2f12d762dea25493085f64fcd24182e4bcfed484ef0 0x8

# Join game
sui client call --package 0x5f234782d0d7fcb5412aafca6e87be1e6b2c67383566f2f4c499bc12ddafb385 --module Game --function enter_game --args 0x813002af527d0803c3bd3a96ba3fc59a829f2e84d21d985cfed64464da57d5d9

# inspect game object
sui client object 0x10e3ef6acb18a493ce5605a26415e07c1fa0f2af3bf233c89da387d7cb716585 --json | jq
```