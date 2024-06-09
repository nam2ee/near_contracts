const { connect, keyStores, WalletConnection, Contract } = window.nearApi;

// NEAR 설정
const nearConfig = {
    networkId: 'testnet',
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://explorer.testnet.near.org',
    contractName: 'past-nail.testnet' // 유효한 NEAR 계정 ID로 업데이트
};

let walletConnection;

// NEAR와 연결 및 계약 초기화
async function initContract() {
    const near = await connect(nearConfig);
    walletConnection = new WalletConnection(near);
    const account = walletConnection.account();

    const contract = new Contract(account, nearConfig.contractName, {
        viewMethods: ['get_all_messages', 'get_user_messages'],
        changeMethods: ['send_message'],
    });

    return { contract, walletConnection };
}

async function login() {
    walletConnection.requestSignIn(
        nearConfig.contractName,
        'NEAR Chat DApp', // title
        '', // successUrl
        '' // failureUrl
    );
}

async function logout() {
    walletConnection.signOut();
    window.location.reload();
}

async function sendMessage() {
    const { contract, walletConnection } = await initContract();

    if (!walletConnection.isSignedIn()) {
        walletConnection.requestSignIn({
            contractId: nearConfig.contractName,
            successUrl: window.location.href,
            failureUrl: window.location.href
        });
    } else {
        const receiver = document.getElementById('receiver').value;
        const content = document.getElementById('content').value;

        await contract.send_message({
            target: receiver,
            content: content
        });

        alert('Message sent!');
    }
}

async function getAllMessages() {
    const { contract } = await initContract();
    const messages = await contract.get_all_messages();

    const messagesList = document.getElementById('messages');
    messagesList.innerHTML = '';

    messages.forEach(message => {
        const li = document.createElement('li');
        li.textContent = `From: ${message.sender}, To: ${message.receiver}, Content: ${message.contents}`;
        messagesList.appendChild(li);
    });
}

async function getUserMessages() {
    const { contract } = await initContract();
    const user = document.getElementById('user').value;
    const messages = await contract.get_user_messages({ user });

    const messagesList = document.getElementById('user-messages');
    messagesList.innerHTML = '';

    messages.forEach(message => {
        const li = document.createElement('li');
        li.textContent = `From: ${message.sender}, To: ${message.receiver}, Content: ${message.contents}`;
        messagesList.appendChild(li);
    });
}

// 전역으로 함수 노출
window.login = login;
window.logout = logout;
window.sendMessage = sendMessage;
window.getAllMessages = getAllMessages;
window.getUserMessages = getUserMessages;

// 로그인 상태에 따라 버튼 표시
document.addEventListener('DOMContentLoaded', async () => {
    const { walletConnection } = await initContract();
    if (walletConnection.isSignedIn()) {
        document.getElementById('login-button').style.display = 'none';
        document.getElementById('logout-button').style.display = 'block';
    } else {
        document.getElementById('login-button').style.display = 'block';
        document.getElementById('logout-button').style.display = 'none';
    }
});
