use std::clone;
use near_sdk::{env, near_bindgen, AccountId, Promise, PanicOnDefault};
use near_sdk::{log, near};


// 메시지 구조체 정의
#[near(serializers = [borsh, json])]
#[derive(Clone)]
pub struct Message {
    sender: AccountId,
    receiver: AccountId,
    contents: String,
}

// 컨트랙트 구조체 정의
#[near(contract_state)]
pub struct ChatContract {
    messages: Vec<Message>,
}

// 기본 구현
impl Default for ChatContract {
    fn default() -> Self {
        Self {
            messages: vec![Message {
                sender: "emptyglove4396.testnet".parse().unwrap(),
                receiver: "broadgrain3282.testnet".parse().unwrap(),
                contents: String::from("Hello near"),
            }],
    }
    }
}

// 컨트랙트 메서드 구현
#[near]
impl ChatContract {
    // 메시지 전송 메서드
    pub fn send_message(&mut self, target:AccountId, content: String) {
        let sender = env::predecessor_account_id();
        let timestamp = env::block_timestamp();

        let message = Message{
            sender: sender,
            receiver: target,
            contents: content,
        };

        // 전체 메시지에 추가
        self.messages.push(message);
    }

    // 모든 메시지 조회 메서드
    pub fn get_all_messages(&self) -> Vec<Message> {
        self.messages.to_vec()
    }

    // 특정 사용자의 메시지 조회 메서드
    pub fn get_user_messages(&self, user: AccountId) -> Vec<Message> {
        self.messages
        .iter()
        .cloned()
        .filter(|msg| msg.receiver == user)
        .collect()
    }
    
    pub fn destroyedself (self) -> i32
    {
        let ChatContract = self;
        7
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}

