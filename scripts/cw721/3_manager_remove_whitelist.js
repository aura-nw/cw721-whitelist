const chainConfig = require('./config/chain').defaultChain;

const fs = require('fs');

const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const { DirectSecp256k1HdWallet, coin } = require('@cosmjs/proto-signing');
const { GasPrice } = require('@cosmjs/stargate');

async function swap(_contract) {
    const deployerWallet = await DirectSecp256k1HdWallet.fromMnemonic(
        chainConfig.deployer_mnemonic,
        {
            prefix: chainConfig.prefix
        }
    );

    // gas price
    const gasPrice = GasPrice.fromString(`0.025${chainConfig.denom}`);

    // connect deployer wallet to chain
    const deployerClient = await SigningCosmWasmClient.connectWithSigner(chainConfig.rpcEndpoint, deployerWallet, {gasPrice});

    // get deployer account
    const deployerAccount = (await deployerWallet.getAccounts())[0];

    const memo = "add whitelist to manager";

    // define the exectute send for cw20
    const executeRemoveWhitelistMsg = {
        "remove_from_whitelist": {
            "addresses": [
                "aura1uh24g2lc8hvvkaaf7awz25lrh5fptthu2dhq0n",
            ],
        }
    }

    console.log("executeMintMsg: ", executeAddWhitelistMsg);

    console.log("deployerAccount.address: ", deployerAccount.address);
    // add whitelist to manager
    const takeResponse = await deployerClient.execute(
        deployerAccount.address,
        _contract,
        executeAddWhitelistMsg,
        "auto",
        memo,
        []
    );

    console.log(takeResponse);
}

const myArgs = process.argv.slice(2);
swap(myArgs[0]);
