INIT='{"token_symbol": "MYT", "token_contract_address": "inj1mgqj43w6f7pfqqfaa9t29gph6gje368ydzwvnc"}'
yes 12345678 | injectived tx wasm instantiate $CODE_ID "$INIT" \
--label="Instantiate Token Vault" \
--from=$(echo $INJ_ADDRESS) \
--chain-id="injective-888" \
--yes --fees=1000000000000000inj \
--gas=2000000 \
--no-admin \
--node=https://testnet.sentry.tm.injective.network:443

DEPOSIT='{"deposit":{"amount":"50"}}'
yes 12345678 | injectived tx wasm execute inj1mssu0cxxdlgumppfpapu2um6m0f635mmjtx8xv "$DEPOSIT" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-888" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node=https://testnet.sentry.tm.injective.network:443 \
--output json