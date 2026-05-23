#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    symbol_short,
    Env,
    String,
    Symbol,
    Vec,
};

// Struktur data chat
#[contracttype]
#[derive(Clone, Debug)]
pub struct ChatMessage {
    id: u64,
    username: String,
    message: String,
}

// Storage key
const CHAT_DATA: Symbol = symbol_short!("CHATDATA");

#[contract]
pub struct ChatContract;

#[contractimpl]
impl ChatContract {

    // Ambil semua chat
    pub fn get_messages(env: Env) -> Vec<ChatMessage> {

        return env
            .storage()
            .instance()
            .get(&CHAT_DATA)
            .unwrap_or(Vec::new(&env));
    }

    // Kirim pesan chat
    pub fn send_message(
        env: Env,
        username: String,
        message: String,
    ) -> String {

        // Ambil data chat lama
        let mut chats: Vec<ChatMessage> = env
            .storage()
            .instance()
            .get(&CHAT_DATA)
            .unwrap_or(Vec::new(&env));

        // Buat object chat baru
        let chat = ChatMessage {
            id: env.prng().gen::<u64>(),
            username: username,
            message: message,
        };

        // Tambahkan chat baru
        chats.push_back(chat);

        // Simpan ke storage
        env.storage()
            .instance()
            .set(&CHAT_DATA, &chats);

        return String::from_str(
            &env,
            "Pesan chat berhasil dikirim"
        );
    }

    // Hapus chat berdasarkan id
    pub fn delete_message(
        env: Env,
        id: u64,
    ) -> String {

        // Ambil data chat
        let mut chats: Vec<ChatMessage> = env
            .storage()
            .instance()
            .get(&CHAT_DATA)
            .unwrap_or(Vec::new(&env));

        // Cari pesan berdasarkan id
        for i in 0..chats.len() {

            if chats.get(i).unwrap().id == id {

                chats.remove(i);

                // Simpan ulang
                env.storage()
                    .instance()
                    .set(&CHAT_DATA, &chats);

                return String::from_str(
                    &env,
                    "Pesan berhasil dihapus"
                );
            }
        }

        return String::from_str(
            &env,
            "Pesan tidak ditemukan"
        );
    }
}

mod test;