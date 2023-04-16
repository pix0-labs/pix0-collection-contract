## Instantiate the contract
```archway instantiate -c 528 --args '{"fees":[{"name":"CREATE_COLLECTION_FEE", "value":{"amount":"1200","denom":"uconst"}},{"name":"CREATE_ITEM_FEE", "value":{"amount":"3200","denom":"uconst"}},{"name":"NFT_MINTING_FEE", "value":{"amount":"2200","denom":"uconst"}},{"name":"SIMPLE_NFT_MINTING_FEE", "value":{"amount":"5400","denom":"uconst"}}] }' ```

## Query the contract info
```archway query contract-state smart --args '{"get_contract_info": {}}'  ```



