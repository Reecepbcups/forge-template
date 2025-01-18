# <h1 align="center"> WavsForge Template </h1>

```bash
# forge this template

forge build

# this is based on ebf8cf6bc001d8292696ef6883d55d749c41bdd9 in the WAVS repo
docker compose up # <- in the WAVS repository


export CLI_EIGEN_CORE_DELEGATION_MANAGER=`docker exec -it wavs bash -c 'jq -r .eigen_core.local.delegation_manager ~/wavs/cli/deployments.json' | tr -d '\r'`
export CLI_EIGEN_CORE_REWARDS_COORDINATOR=`docker exec -it wavs bash -c 'jq -r .eigen_core.local.rewards_coordinator ~/wavs/cli/deployments.json' | tr -d '\r'`
export CLI_EIGEN_CORE_AVS_DIRECTORY=`docker exec -it wavs bash -c 'jq -r .eigen_core.local.avs_directory ~/wavs/cli/deployments.json' | tr -d '\r'`
export FOUNDRY_ANVIL_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

forge script ./script/WavsServiceManager.s.sol --rpc-url http://localhost:8545 --broadcast
```

Custom component
```bash
```
