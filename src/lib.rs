#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, log};

#[contracttype]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    Allowance(Address, Address),
    Metadata,
    TotalSupply,
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(env: Env, admin: Address, name: String, symbol: String, decimals: u32) {
        // Storage kontrolü
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("sözleşme zaten başlatıldı");
        }
        
        // Admin'i ayarla
        env.storage().instance().set(&DataKey::Admin, &admin);
        
        // Meta verileri ayarla
        let metadata = TokenMetadata { name, symbol, decimals };
        env.storage().instance().set(&DataKey::Metadata, &metadata);
        
        // Başlangıç toplam arzı
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
    }

    pub fn mint(env: Env, admin: Address, to: Address, amount: i128) {
        // Admin kontrolü
        let current_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if current_admin != admin {
            panic!("yalnızca yönetici");
        }
        
        if amount <= 0 {
            panic!("miktarın pozitif olması gerekiyor");
        }

        // Bakiye güncelleme
        let balance: i128 = env.storage().instance().get(&DataKey::Balance(to.clone())).unwrap_or(0);
        let new_balance = balance.checked_add(amount).unwrap();
        env.storage().instance().set(&DataKey::Balance(to.clone()), &new_balance);

        // Toplam arz güncelleme
        let total_supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        let new_supply = total_supply.checked_add(amount).unwrap();
        env.storage().instance().set(&DataKey::TotalSupply, &new_supply);
        
        log!(&env, "Token basma başarılı, alıcı: {}, miktar: {}", to, amount);
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        if amount <= 0 {
            panic!("miktarın pozitif olması gerekiyor");
        }

        // Bakiyeleri kontrol et ve güncelle
        let from_balance: i128 = env.storage().instance().get(&DataKey::Balance(from.clone())).unwrap_or(0);
        if from_balance < amount {
            panic!("yetersiz bakiye");
        }

        let to_balance: i128 = env.storage().instance().get(&DataKey::Balance(to.clone())).unwrap_or(0);
        
        let new_from_balance = from_balance.checked_sub(amount).unwrap();
        let new_to_balance = to_balance.checked_add(amount).unwrap();
        
        env.storage().instance().set(&DataKey::Balance(from.clone()), &new_from_balance);
        env.storage().instance().set(&DataKey::Balance(to.clone()), &new_to_balance);
        
        log!(&env, "Transfer başarılı, gönderen: {}, alıcı: {}, miktar: {}", from, to, amount);
    }

    pub fn balance(env: Env, addr: Address) -> i128 {
        env.storage().instance().get(&DataKey::Balance(addr)).unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
    }

    pub fn approve(env: Env, from: Address, spender: Address, amount: i128) {
        from.require_auth();

        if amount < 0 {
            panic!("miktarın negatif olmaması gerekiyor");
        }

        if amount > 0 {
            env.storage().instance().set(&DataKey::Allowance(from.clone(), spender.clone()), &amount);
        } else {
            env.storage().instance().remove(&DataKey::Allowance(from.clone(), spender.clone()));
        }
        
        log!(&env, "Harcama yetkisi verildi: {} -> {}, miktar: {}", from, spender, amount);
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage().instance().get(&DataKey::Allowance(from.clone(), spender.clone())).unwrap_or(0)
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        if amount <= 0 {
            panic!("miktarın pozitif olması gerekiyor");
        }

        // Bakiye ve izin kontrolü
        let from_balance: i128 = env.storage().instance().get(&DataKey::Balance(from.clone())).unwrap_or(0);
        if from_balance < amount {
            panic!("yetersiz bakiye");
        }

        let spender_allowance: i128 = env.storage().instance().get(&DataKey::Allowance(from.clone(), spender.clone())).unwrap_or(0);
        if spender_allowance < amount {
            panic!("yetersiz izin");
        }

        // Bakiyeleri güncelle
        let to_balance: i128 = env.storage().instance().get(&DataKey::Balance(to.clone())).unwrap_or(0);
        
        let new_from_balance = from_balance.checked_sub(amount).unwrap();
        let new_to_balance = to_balance.checked_add(amount).unwrap();
        
        env.storage().instance().set(&DataKey::Balance(from.clone()), &new_from_balance);
        env.storage().instance().set(&DataKey::Balance(to.clone()), &new_to_balance);

        // İzni güncelle
        let new_allowance = spender_allowance.checked_sub(amount).unwrap();
        env.storage().instance().set(&DataKey::Allowance(from.clone(), spender.clone()), &new_allowance);
        
        log!(&env, "Transfer_from başarılı, harcayıcı: {}, gönderen: {}, alıcı: {}, miktar: {}", 
            spender, from, to, amount);
    }

    pub fn get_metadata(env: Env) -> TokenMetadata {
        env.storage().instance().get(&DataKey::Metadata).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Bytes, IntoVal};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let admin = Address::generate(&env);
        
        // Yeni oluşturma yöntemi
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);
        
        let name = String::from_str(&env, "Soroban Token");
        let symbol = String::from_str(&env, "SRT");
        
        client.initialize(&admin, &name, &symbol, &8);
        
        let metadata = client.get_metadata();
        assert_eq!(metadata.name, name);
        assert_eq!(metadata.symbol, symbol);
        assert_eq!(metadata.decimals, 8);
    }

    #[test]
    fn test_mint_and_balance() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        
        // Yeni oluşturma yöntemi
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);
        
        client.initialize(
            &admin, 
            &String::from_str(&env, "Soroban Token"),
            &String::from_str(&env, "SRT"),
            &8
        );
        
        client.mint(&admin, &user, &100);
        assert_eq!(client.balance(&user), 100);
        assert_eq!(client.total_supply(), 100);
    }

    #[test]
    fn test_transfer() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        
        // Yeni oluşturma yöntemi
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);
        
        client.initialize(
            &admin, 
            &String::from_str(&env, "Soroban Token"),
            &String::from_str(&env, "SRT"),
            &8
        );
        
        client.mint(&admin, &user1, &100);
        
        env.mock_all_auths();
        
        client.transfer(&user1, &user2, &30);
        
        assert_eq!(client.balance(&user1), 70);
        assert_eq!(client.balance(&user2), 30);
    }
}