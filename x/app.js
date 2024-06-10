const nearAPI = require('near-api-js');
const { connect, keyStores, WalletConnection } = nearAPI;

const CONTRACT_NAME = 'your-contract-name.testnet'; // 여기에 실제 컨트랙트 이름을 넣으세요

async function initContract() {
    const nearConfig = {
        networkId: 'testnet',
        keyStore: new keyStores.BrowserLocalStorageKeyStore(),
        nodeUrl: 'https://rpc.testnet.near.org',
        walletUrl: 'https://wallet.testnet.near.org',
        helperUrl: 'https://helper.testnet.near.org',
        explorerUrl: 'https://explorer.testnet.near.org',
    };

    const near = await connect(nearConfig);
    const wallet = new WalletConnection(near);

    return {
        wallet,
        contract: new nearAPI.Contract(wallet.account(), CONTRACT_NAME, {
            viewMethods: ['get_all_messages', 'get_user_messages'],
            changeMethods: ['send_message', 'destroy_self'],
        }),
    };
}

async function sendMessage(contract, target, content) {
    await contract.send_message({
        target,
        content,
    });
}

async function getAllMessages(contract) {
    return await contract.get_all_messages();
}

async function getUserMessages(contract, user) {
    return await contract.get_user_messages({ user });
}

async function destroySelf(contract) {
    await contract.destroy_self();
}

document.addEventListener('DOMContentLoaded', async () => {
    const { wallet, contract } = await initContract();

    if (!wallet.isSignedIn()) {
        wallet.requestSignIn(CONTRACT_NAME);
    }

    document.getElementById('sendMessageButton').onclick = async () => {
        const target = document.getElementById('receiver').value;
        const content = document.getElementById('message').value;
        await sendMessage(contract, target, content);
    };

    document.getElementById('getAllMessagesButton').onclick = async () => {
        const messages = await getAllMessages(contract);
        document.getElementById('messages').innerText = JSON.stringify(messages, null, 2);
    };

    document.getElementById('getUserMessagesButton').onclick = async () => {
        const user = document.getElementById('account').value;
        const messages = await getUserMessages(contract, user);
        document.getElementById('messages').innerText = JSON.stringify(messages, null, 2);
    };

    document.getElementById('destroySelfButton').onclick = async () => {
        await destroySelf(contract);
    };
});
