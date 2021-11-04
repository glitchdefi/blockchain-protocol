# SET UP NODE GLITCH

### BUILD SOURCE
```angular2html
 cargo build --release
```

### FULL NODE

```angular2html
./target/release/glitch-node \
  --base-path /tmp/fullnode \
  --chain ./testnetGenesisRaw.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --name FullNode \
  --ws-external \
  --rpc-external \

```

### ARCHIVE NODE

```angular2html
./target/release/glitch-node \
  --base-path /tmp/fullnode \
  --chain ./testnetGenesisRaw.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --name ArchiveNode \
  --pruning archive \
```

### VALIDATOR NODE
First, run node with this command

```angular2html
./target/release/glitch-node \
  --base-path /tmp/fullnode \
  --chain ./testnetGenesisRaw.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --name ValidatorNode \
  --ws-external \
  --rpc-external \
  --validator
```
Then, generate key with curl 
```angular2html
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' http://localhost:9933/
```
Take result and add to stash account . You've already been validator (It takes some minute to vote)





