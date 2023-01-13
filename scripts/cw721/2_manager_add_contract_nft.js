const chainConfig = require('./config/chain').defaultChain;

const fs = require('fs');

const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const { DirectSecp256k1HdWallet, coin } = require('@cosmjs/proto-signing');
const { GasPrice } = require('@cosmjs/stargate');

async function swap(_contract) {
    const testerWallet = await DirectSecp256k1HdWallet.fromMnemonic(
        chainConfig.tester_mnemonic,
        {
            prefix: chainConfig.prefix
        }
    );

    // gas price
    const gasPrice = GasPrice.fromString(`0.025${chainConfig.denom}`);

    // connect tester wallet to chain
    const testerClient = await SigningCosmWasmClient.connectWithSigner(chainConfig.rpcEndpoint, testerWallet, {gasPrice});

    // get tester account
    const testerAccount = (await testerWallet.getAccounts())[0];

    const memo = "convert from native to vaura";

    // define the set manager send for cw20
    const executeMintMsg = {
        "mint": {
            "recipient": testerAccount.address,
            "amount": "10000000",
        }
    }

    console.log("executeMintMsg: ", executeMintMsg);

    console.log("testerAccount.address: ", testerAccount.address);
    // send the cw20 token to contract
    const takeResponse = await testerClient.execute(
        testerAccount.address, 
        _contract, 
        executeMintMsg, 
        "auto", 
        memo, 
        [coin(10000000, chainConfig.denom)]
    );

    console.log(takeResponse);
}

const myArgs = process.argv.slice(2);
swap(myArgs[0]);
