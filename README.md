# <h1 align="center"> WavsForge Template </h1>

```bash
# forge this template

forge build

# this is based on ebf8cf6bc001d8292696ef6883d55d749c41bdd9 in the WAVS repo
docker compose up # <- in the WAVS repository

# used later to read from
mkdir -p .docker; docker cp wavs:/root/wavs/cli/deployments.json .docker/deployments.json

export CLI_EIGEN_CORE_DELEGATION_MANAGER=`docker exec -it wavs bash -c 'jq -r .eigen_core.local.delegation_manager ~/wavs/cli/deployments.json' | tr -d '\r'`
export CLI_EIGEN_CORE_REWARDS_COORDINATOR=`docker exec -it wavs bash -c 'jq -r .eigen_core.local.rewards_coordinator ~/wavs/cli/deployments.json' | tr -d '\r'`
export CLI_EIGEN_CORE_AVS_DIRECTORY=`docker exec -it wavs bash -c 'jq -r .eigen_core.local.avs_directory ~/wavs/cli/deployments.json' | tr -d '\r'`
export FOUNDRY_ANVIL_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

forge script ./script/WavsServiceManager.s.sol --rpc-url http://localhost:8545 --broadcast
```

# Custom component
```bash
(cd components/eth-trigger-echo; cargo component build --release)

# TODO: I do not like this output dir name. Can't use `out` (solidity) or `components`.
mkdir -p ./compiled && cp ./components/target/wasm32-wasip1/release/*.wasm ./compiled/
```

# Verify
```bash
# install wavs-cli
# https://github.com/Lay3rLabs/WAVS/issues/277
# - right now `E0308` is super annoying @ packages/wavs/src/submission/core.rs:426:13 ^ have to do a `result: result.into(),` for now.
(cd lib/WAVS; cargo install --path ./packages/cli)

cp ./lib/WAVS/packages/cli/wavs-cli.toml .

# Local Exec
# ./compiled/eth_trigger_echo.wasm does not work because it's relative in `read_component(path: impl AsRef<Path>) -> Vec<u8> ...  Path::new("../../")`
wavs-cli exec --component $(pwd)/compiled/eth_trigger_echo.wasm --input testing


# Actual deploy
# Service manager is from the script/WavsServiceManager.s.sol broadcast logs
export WAVS_CLI_ETH_MNEMONIC="test test test test test test test test test test test junk"
wavs-cli deploy-service --component $(pwd)/compiled/eth_trigger_echo.wasm  --service-manager 0x851356ae760d987E095750cCeb3bC6014560891C --data ./.docker

wavs-cli add-task --service-id 019476fe0fe27d60af2a2af6212ca38a --input "hello testing!" --data ./.docker
```
