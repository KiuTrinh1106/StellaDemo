#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, String, Vec,
};

// ── Storage keys ──────────────────────────────────────────
#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    TotalMinted,
}

// ── Contract ──────────────────────────────────────────────
#[contract]
pub struct UniTokenContract;

#[contractimpl]
impl UniTokenContract {

    // Khởi tạo contract, set admin
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TotalMinted, &0u64);
    }

    // Admin mint token cho sinh viên sau khi điểm danh
    pub fn mint(env: Env, student: Address, amount: u64) {
        // Xác thực admin
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin).unwrap();
        admin.require_auth();

        // Cộng token vào số dư sinh viên
        let current: u64 = env.storage().persistent()
            .get(&DataKey::Balance(student.clone()))
            .unwrap_or(0);
        env.storage().persistent()
            .set(&DataKey::Balance(student.clone()), &(current + amount));

        // Cập nhật tổng token đã mint
        let total: u64 = env.storage().instance()
            .get(&DataKey::TotalMinted).unwrap_or(0);
        env.storage().instance()
            .set(&DataKey::TotalMinted, &(total + amount));

        // Emit event để frontend lắng nghe
        env.events().publish(
            (symbol_short!("mint"),),
            (student, amount),
        );
    }

    // Sinh viên đổi token lấy quyền lợi
    pub fn redeem(env: Env, student: Address, amount: u64) {
        student.require_auth();

        let current: u64 = env.storage().persistent()
            .get(&DataKey::Balance(student.clone()))
            .unwrap_or(0);

        if current < amount {
            panic!("Insufficient token balance");
        }

        env.storage().persistent()
            .set(&DataKey::Balance(student.clone()), &(current - amount));

        env.events().publish(
            (symbol_short!("redeem"),),
            (student, amount),
        );
    }

    // Query số dư token của sinh viên
    pub fn balance(env: Env, student: Address) -> u64 {
        env.storage().persistent()
            .get(&DataKey::Balance(student))
            .unwrap_or(0)
    }

    // Query tổng token đã phát hành
    pub fn total_minted(env: Env) -> u64 {
        env.storage().instance()
            .get(&DataKey::TotalMinted)
            .unwrap_or(0)
    }

    // Admin chuyển quyền admin sang địa chỉ khác
    pub fn transfer_admin(env: Env, new_admin: Address) {
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }
}

#[cfg(test)]
mod test;
