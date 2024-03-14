build:
	cargo wasm

optimize:
	docker run --rm -v "$$(pwd)":/code \
		--mount type=volume,source="$$(basename "$$(pwd)")_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.14.0
test:
	cargo unit-test

upload-testnet:
	seid tx wasm store ./artifacts/raffle.wasm -y --from=dj --chain-id=atlantic-2 --node https://rpc.atlantic-2.seinetwork.io --gas=10000000 --fees=1000000usei --broadcast-mode=block

instantiate-testnet:
	seid tx wasm instantiate ${id} '{"count": 5, "owner": "sei1j7ah3st8qjr792qjwtnjmj65rqhpedjqf9dnsd"}' --chain-id atlantic-2 --from dj --gas=4000000 --fees=1000000usei --broadcast-mode=block --label raffle --no-admin --node https://rpc.atlantic-2.seinetwork.io

balance-hk-testnet:
	seid q bank balances sei1cz56s8l9yz92jgstv9y4pyxj8vkdnw7acug8n7 --node https://rpc.atlantic-2.seinetwork.io --chain-id atlantic-2

launch-token:
	seid tx tokenfactory create-denom ${name} --from sei1cz56s8l9yz92jgstv9y4pyxj8vkdnw7acug8n7 --node https://rpc.atlantic-2.seinetwork.io --chain-id atlantic-2 --gas=200000 --fees=2000000usei -b block -y

mint-token:
	seid tx tokenfactory mint ${amount} --from sei1cz56s8l9yz92jgstv9y4pyxj8vkdnw7acug8n7 --chain-id atlantic-2 -b block -y --node https://rpc.atlantic-2.seinetwork.io --gas=200000 --fees=2000000usei

# 1000factory/sei1cz56s8l9yz92jgstv9y4pyxj8vkdnw7acug8n7/sdasdf

burn-token:
	seid tx tokenfactory burn ${amount} --from hk --chain-id atlantic-2

list-token:
	seid q tokenfactory denoms-from-creator sei1cz56s8l9yz92jgstv9y4pyxj8vkdnw7acug8n7 --chain-id atlantic-2 --node https://rpc.atlantic-2.seinetwork.io

send-token:
	seid tx bank send sei1cz56s8l9yz92jgstv9y4pyxj8vkdnw7acug8n7 sei1g0g6kr73egtysamh6zp6cleqzdafcreyxapgklgnwg49qnveh6rst08jst 100factory/sei1cz56s8l9yz92jgstv9y4pyxj8vkdnw7acug8n7/ktg --chain-id=atlantic-2 -b block -y --node https://rpc.atlantic-2.seinetwork.io --gas=200000 --fees=2000000usei

balance-testnet:
	seid q bank balances ${address} --node https://rpc.atlantic-2.seinetwork.io --chain-id atlantic-2

prepare:
	export GOPATH=~/go && export PATH=$PATH:$GOPATH/bin
