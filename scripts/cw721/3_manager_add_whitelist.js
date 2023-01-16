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
    const executeAddWhitelistMsg = {
        "add_to_whitelist": {
            "whitelist_infos": [
                {
                    "address": "aura1uh24g2lc8hvvkaaf7awz25lrh5fptthu2dhq0n",
                    "uri": "https://ipfs.io/ipfs/QmV8cx4TAMX4ghZJTXKFYG37Fq4uLmqXrKNB1jUTXqke3R/images/3117",
                },
                {
                    "address": "aura1fqj2redmssckrdeekhkcvd2kzp9f4nks4fctrt",
                    "uri": "https://ipfs.io/ipfs/QmTvCyRe4DqGNFCMEcy4jDjjVewPpqqnK7BbVnfegozd13/4202.json",
                },
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
