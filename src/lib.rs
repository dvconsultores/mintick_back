
use near_contract_standards::non_fungible_token::core::{
    NonFungibleTokenCore, NonFungibleTokenResolver,
};
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};


use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue, Balance,
    serde_json::json, assert_one_yocto,
};
use near_sdk::collections::{LazyOption, UnorderedMap, UnorderedSet};

use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;
use near_sdk::env::is_valid_account_id;

near_sdk::setup_alloc!();



pub type TokenSeriesId = String;



pub const TOKEN_DELIMETER: char = ':';
pub const TITLE_DELIMETER: &str = " #";
pub const VAULT_FEE: u128 = 500;


const MAX_PRICE: Balance = 1_000_000_000 * 10u128.pow(24);
/*
const GAS_FOR_RESOLVE_TRANSFER: Gas = 10_000_000_000_000;
const GAS_FOR_NFT_TRANSFER_CALL: Gas = 30_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER;
const GAS_FOR_NFT_APPROVE: Gas = 10_000_000_000_000;
const GAS_FOR_MINT: Gas = 90_000_000_000_000;
const NO_DEPOSIT: Balance = 0;
*/


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CategoriesObjet {
	name: String,
    img: String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CategoriesJson {
    id: i128,
	name: String,
    img: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MarketJson {
    token_series_id: TokenSeriesId,
    metadata: TokenMetadata,
    owner_id: AccountId,
    creator_id: AccountId,
    price: Balance,
    category: HashMap<i128, CategoriesObjet>,
    royalty: HashMap<AccountId, u32>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MarketView {
    token_series_id: TokenSeriesId,
    metadata: TokenMetadata,
    owner_id: AccountId,
    creator_id: AccountId,
    price: Balance,
    category: HashMap<i128, CategoriesObjet>,
    royalty: HashMap<AccountId, u32>,
    reviews: Vec<Review>,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TransactionJson {
    token_id: TokenSeriesId,
    operations: i128,
    sales: i128,
    operation_history: Vec<OperationHistory>,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OperationHistory {
    owner_id: AccountId,
    price: Balance,
    date: u64,
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenSeries {
	metadata: TokenMetadata,
	creator_id: AccountId,
	tokens: UnorderedSet<TokenId>,
    price: Option<Balance>,
    is_mintable: bool,
    category: HashMap<i128, CategoriesObjet>,
    royalty: HashMap<AccountId, u32>,
    reviews: UnorderedSet<Review>,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Review {
    user_id: AccountId,
    review: String,
    critics: i8,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenSeriesJson {
    token_series_id: TokenSeriesId,
	metadata: TokenMetadata,
	creator_id: AccountId,
    royalty: HashMap<AccountId, u32>
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    // CUSTOM
    token_series_by_id: UnorderedMap<TokenSeriesId, TokenSeries>,
    vault_id: AccountId,
    categories: Vec<CategoriesJson>,
    marketplace: UnorderedMap<TokenSeriesId, MarketJson>,
    transaction: UnorderedMap<TokenSeriesId, TransactionJson>,
    administrators: Vec<AccountId>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";


#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    // CUSTOM
    TokenSeriesById,
    TokensBySeriesInner { token_series: String },
    TokensPerOwner { account_hash: Vec<u8> },
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: ValidAccountId, vault_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            vault_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "MinTick".to_string(),
                symbol: "MinTick".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: ValidAccountId, vault_id: ValidAccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            token_series_by_id: UnorderedMap::new(StorageKey::TokenSeriesById),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            vault_id: vault_id.to_string(),
            profile: UnorderedMap::new(b"s".to_vec()),
            categories: Vec::new(),
            marketplace: UnorderedMap::new(b"0".to_vec()),
            transaction: UnorderedMap::new(b"s".to_vec()),
            administrators: vec!["mintick.testnet".to_string(), "min.mintick.testnet".to_string()],
        }
    }

    pub fn set_admin(&mut self, user_id: AccountId) {      
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators can set categories");
        let valid = self.administrators.iter().find(|&x| x == &user_id);
        if valid.is_some() {
            env::panic(b"the user is already in the list of administrators");
        }
        self.administrators.push(user_id);
    }

    pub fn delete_admin(&mut self, user_id: AccountId) {      
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators can set categories");
        let index = self.administrators.iter().position(|x| x == &user_id.to_string()).expect("the user is not in the list of administrators");
        self.administrators.remove(index);
    }


    pub fn set_category(&mut self, name: String, img: String) -> CategoriesJson {      
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators can set categories");
        let category_id: i128 = (self.categories.len() + 1) as i128;
        let data = CategoriesJson {
            id: category_id,
            name: name.to_string(),
            img: img.to_string(),
        };
        
        self.categories.push(data.clone());
        env::log(b"category Created");
        
        data
    }

    pub fn put_category(&mut self, category_id: i128, name: String, img: String) -> CategoriesJson {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only admins can edit categories");
        let index = self.categories.iter().position(|x| x.id == category_id).expect("Category does not exist");
        self.categories[index].name = name.to_string();
        self.categories[index].img = img.to_string();

        env::log(b"Category Update");

        CategoriesJson {
            id: category_id,
            name: name.to_string(),
            img: img.to_string(),
        }
    }

    pub fn get_category(&self, category_id: Option<i128>) -> Vec<CategoriesJson> {
        let mut categories = self.categories.clone();

        if category_id.is_some() {
            categories = self.categories.iter().filter(|x| x.id == category_id.unwrap()).map(|x| CategoriesJson {
                id: x.id,
                name: x.name.to_string(),
                img: x.img.to_string(),
            }).collect();
        }
        categories
    }


    #[payable]
    pub fn nft_series(
        &mut self,
        token_metadata: TokenMetadata,
        category: Vec<i128>,
        price: Option<U128>,
        royalty: Option<HashMap<AccountId, u32>>,
    ) -> TokenSeriesJson {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only admins");
        let initial_storage_usage = env::storage_usage();
        let caller_id = env::signer_account_id();

        let token_series_id = format!("{}", (self.token_series_by_id.len() + 1));

        assert!(
            self.token_series_by_id.get(&token_series_id).is_none(),
            "duplicate token_series_id"
        );

        let title = token_metadata.title.clone();
        assert!(title.is_some(), "token_metadata.title is required");
        

        let mut total_perpetual = 0;
        let mut total_accounts = 0;
        let royalty_res: HashMap<AccountId, u32> = if let Some(royalty) = royalty {
            for (k , v) in royalty.iter() {
                if !is_valid_account_id(k.as_bytes()) {
                    env::panic("Not valid account_id for royalty".as_bytes());
                };
                total_perpetual += *v;
                total_accounts += 1;
            }
            royalty
        } else {
            HashMap::new()
        };

        assert!(total_accounts <= 10, "royalty exceeds 10 accounts");

        assert!(
            total_perpetual <= 9000,
            "Exceeds maximum royalty -> 9000",
        );

        let price_res: Option<u128> = if price.is_some() {
            assert!(
                price.unwrap().0 < MAX_PRICE,
                "price higher than {}",
                MAX_PRICE
            );
            Some(price.unwrap().0)
        } else {
            None
        };

        //let category_res: Vec<i128> = category.clone();
        let mut categorys: HashMap<i128, CategoriesObjet> = HashMap::new();
        
        category.iter().for_each(|x| {
            let index = self.categories.iter().position(|z| z.id == *x).expect("Category does not exist");
            categorys.insert(*x, CategoriesObjet {
                name: self.categories[index].name.clone(),
                img: self.categories[index].img.clone(),
            });
        });

        if price.is_some() {
            self.marketplace.insert(&token_series_id, &MarketJson {
                token_series_id: token_series_id.to_string(),
                metadata: token_metadata.clone(),
                owner_id: caller_id.to_string(),
                creator_id: caller_id.to_string(),
                price: price.unwrap().0,
                category: categorys.clone(),
                royalty: royalty_res.clone(),
            });
        }


        self.token_series_by_id.insert(&token_series_id, &TokenSeries{
            metadata: token_metadata.clone(),
            creator_id: caller_id.to_string(),
            tokens: UnorderedSet::new(
                StorageKey::TokensBySeriesInner {
                    token_series: token_series_id.clone(),
                }
                .try_to_vec()
                .unwrap(),
            ),
            price: price_res,
            is_mintable: true,
            category: categorys.clone(),
            royalty: royalty_res.clone(),
            reviews: UnorderedSet::new(b"s".to_vec()),
        });

        env::log(
            json!({
                "type": "nft_create_series",
                "params": {
                    "token_series_id": token_series_id,
                    "token_metadata": token_metadata,
                    "creator_id": caller_id,
                    "price": price,
                    "royalty": royalty_res
                }
            })
            .to_string()
            .as_bytes(),
        );

        refund_deposit(env::storage_usage() - initial_storage_usage, 0);

		TokenSeriesJson{
            token_series_id,
			metadata: token_metadata,
			creator_id: caller_id.into(),
            royalty: royalty_res,
		}
    }

    

    #[payable]
    pub fn nft_mint_series(
        &mut self, 
        token_series_id: TokenSeriesId, 
        receiver_id: ValidAccountId
    ) -> TokenId {
        let initial_storage_usage = env::storage_usage();

        let token = token_series_id.clone();

        let token_series = self.token_series_by_id.get(&token_series_id).expect("Token series not exist");
        assert_eq!(env::predecessor_account_id(), token_series.creator_id, "not creator");
        let token_id: TokenId = self._nft_mint_series(token_series_id, receiver_id.to_string());
        
        
        // self.transaction_add(token, receiver_id.to_string(), 0);


        refund_deposit(env::storage_usage() - initial_storage_usage, 0);

        token_id
    }


    #[payable]
    pub fn put_nft_series_price(&mut self, token_series_id: TokenSeriesId
        , price: Option<U128>
    ) -> Option<U128> {
        assert_one_yocto();
        let mut owner_by_id: Option<AccountId> = None;
        let mut token_id: TokenSeriesId = token_series_id.clone(); 
        let mut category: HashMap<i128, CategoriesObjet> = HashMap::new();
        match token_series_id.split(TOKEN_DELIMETER).collect::<Vec<&str>>().len() {
            1=> {
                    token_id = token_series_id.clone();
                    let mut token_series = self.token_series_by_id.get(&token_id).expect("Token series not exist");
                    assert_eq!(
                        env::predecessor_account_id(),
                        token_series.creator_id,
                        "Creator only"
                    );
                    owner_by_id = Some(token_series.creator_id.clone());
                    category = token_series.category.clone();
                    assert_eq!(
                        token_series.is_mintable,
                        true,
                        "token series is not mintable"
                    );
                    
                    if price.is_none() {
                        token_series.price = None;
                    } else {
                        assert!(
                            price.unwrap().0 < MAX_PRICE,
                            "price higher than {}",
                            MAX_PRICE
                        );
                        token_series.price = Some(price.unwrap().0);
                    }
            
                    self.token_series_by_id.insert(&token_id, &token_series);
                },
            2=> {
                    token_id = token_series_id.split(TOKEN_DELIMETER).collect::<Vec<&str>>()[0].to_string();
                    let token_series = self.token_series_by_id.get(&token_id).expect("Token series not exist");
                    category = token_series.category;
                    let owner_id = self.tokens.owner_by_id.get(&token_series_id).expect("No token id");
                    owner_by_id = Some(owner_id);
                    
                    
                },
            _=> env::panic(b"token_series_id invalid"),
        };

        if owner_by_id.is_some() {
            let tokenseries = self.token_series_by_id.get(&token_id).expect("Token series not exist");
        
            let token_metadata = tokenseries.metadata.clone();
            let caller_id = tokenseries.creator_id.clone();
            let royalty_res = tokenseries.royalty.clone();

            if price.is_none() {
                if self.marketplace.get(&token_series_id).is_some() {
                    self.marketplace.remove(&token_series_id);
                };
            } else {
                assert!(
                    price.unwrap().0 < MAX_PRICE,
                    "price higher than {}",
                    MAX_PRICE
                );
                if self.marketplace.get(&token_series_id).is_some() {
                    let mut market = self.marketplace.get(&token_series_id).expect("error");
                    market.price = price.unwrap().0;
                    self.marketplace.insert(&token_series_id, &market);
                } else {
                    self.marketplace.insert(&token_series_id, &MarketJson {
                        token_series_id: token_series_id.to_string(),
                        metadata: token_metadata.clone(),
                        owner_id: owner_by_id.unwrap(),
                        creator_id: caller_id.to_string(),
                        price: price.unwrap().0,
                        category: category,
                        royalty: royalty_res.clone(),
                    });
                };
            }
        } else {
            env::panic(b"token_series_id invalid");
        };
        price
    }
    
    #[payable]
    pub fn nft_buy(
        &mut self, 
        token_series_id: TokenSeriesId, 
        receiver_id: ValidAccountId
    ) -> TokenId {
        let initial_storage_usage = env::storage_usage();
        let mut token_id: TokenId =  "-".to_string();
        match token_series_id.split(TOKEN_DELIMETER).collect::<Vec<&str>>().len() {
            1=> {
                    let token_series = self.token_series_by_id.get(&token_series_id).expect("Token series not exist");
                    let price: u128 = token_series.price.expect("not for sale");
                    let attached_deposit = env::attached_deposit();
                    assert!(
                        attached_deposit >= price,
                        "attached deposit is less than price : {}",
                        price
                    );

                    let mut profile = self.profile.get(&token_series.creator_id).expect("Profile does not exist");
                    profile.sales = profile.sales + 1;
                    self.profile.insert(&token_series.creator_id, &profile);

                    // self.transaction_add(token_series_id.clone(), receiver_id.to_string(), price);

                    token_id = self._nft_mint_series(token_series_id, receiver_id.to_string());

                    
            
                    let for_vault = price as u128 * VAULT_FEE / 10_000u128;
                    let price_deducted = price - for_vault;
                    Promise::new(token_series.creator_id).transfer(price_deducted);
                    Promise::new(self.vault_id.clone()).transfer(for_vault);
            
                    refund_deposit(env::storage_usage() - initial_storage_usage, price);
            
                },
            2=> {
                    let token_data = self.marketplace.get(&token_series_id).expect("Token not for sale");
                    let price: u128 = token_data.price;
                    let attached_deposit = env::attached_deposit();
                    assert!(
                        attached_deposit >= price,
                        "attached deposit is less than price : {}",
                        price
                    );

                    let mut profile = self.profile.get(&token_data.owner_id).expect("Profile does not exist");
                    profile.sales = profile.sales + 1;
                    self.profile.insert(&token_data.owner_id, &profile);

                    // self.transaction_add(token_series_id.clone(), receiver_id.to_string(), price);
                    
                    self.tokens.nft_transfer(receiver_id.clone(), token_series_id.clone(), None, None);

                    let for_vault = price as u128 * VAULT_FEE / 10_000u128;
                    let price_deducted = price - for_vault;
                    Promise::new(token_data.owner_id).transfer(price_deducted);
                    Promise::new(self.vault_id.clone()).transfer(for_vault);
            
                    refund_deposit(env::storage_usage() - initial_storage_usage, price);
                    
                    token_id = token_series_id.to_string().clone();
                            
                },
            _=> env::panic(b"token_series_id invalid"),
        };

        token_id
    }


    fn _nft_mint_series(
        &mut self, 
        token_series_id: TokenSeriesId, 
        receiver_id: AccountId
    ) -> TokenId {
        let mut token_series = self.token_series_by_id.get(&token_series_id).expect("Token series not exist");
        let metadata: TokenMetadata = token_series.metadata.clone();
        assert!(
            token_series.is_mintable,
            "Token series is not mintable"
        );

        let num_tokens = token_series.tokens.len();
        let max_copies = token_series.metadata.copies.unwrap_or(u64::MAX);
        assert!(num_tokens < max_copies, "Series supply maxed");

        if (num_tokens + 1) >= max_copies {
            token_series.is_mintable = false;
            token_series.price = None;
        }
        
        let token_id = format!("{}{}{}", &token_series_id, TOKEN_DELIMETER, num_tokens + 1);
        token_series.tokens.insert(&token_id);
        self.token_series_by_id.insert(&token_series_id, &token_series);
        let title: String = format!("{}{}{}{}{}", token_series.metadata.title.unwrap().clone(), TITLE_DELIMETER, &token_series_id, TITLE_DELIMETER, (num_tokens + 1).to_string());
        
        
        let metadata = TokenMetadata {
            title: Some(title),          
            description: token_series.metadata.description.clone(),   
            media: token_series.metadata.media.clone(),
            media_hash: token_series.metadata.media_hash, 
            copies: token_series.metadata.copies, 
            issued_at: Some(env::block_timestamp().to_string()), 
            expires_at: token_series.metadata.expires_at,
            starts_at: token_series.metadata.starts_at, 
            updated_at: token_series.metadata.updated_at,
            extra: token_series.metadata.extra.clone(),
            reference: token_series.metadata.reference.clone(),
            reference_hash: token_series.metadata.reference_hash,
        };

        let owner_id: AccountId = receiver_id;
        self.tokens.owner_by_id.insert(&token_id, &owner_id);

        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &metadata));

         if let Some(tokens_per_owner) = &mut self.tokens.tokens_per_owner {
             let mut token_ids = tokens_per_owner.get(&owner_id).unwrap_or_else(|| {
                 UnorderedSet::new(StorageKey::TokensPerOwner {
                     account_hash: env::sha256(&owner_id.as_bytes()),
                 })
             });
             token_ids.insert(&token_id);
             tokens_per_owner.insert(&owner_id, &token_ids);
         };

        token_id
    }

    fn transaction_add(&mut self, token_id: TokenSeriesId, 
        owner_id: AccountId,
        price: Balance
    ) -> bool {
        let mut result = false;
        self.token_series_by_id.get(&token_id).expect("Token series not exist");
        let mut sales = 0;
        let mut final_price: u128 = "0".parse::<u128>().unwrap();

        if price > 0 {
            final_price = price;
            sales = 1;
        };


        if self.transaction.get(&token_id).is_some() {
            let mut trans = self.transaction.get(&token_id).expect("Token series not exist");
            trans.operations = trans.operations + 1;
            if price > 0 {
                trans.sales = trans.sales + 1;
            };
            trans.operation_history.push(OperationHistory {
                owner_id: owner_id.to_string(),
                price: final_price,
                date: env::block_timestamp().to_string().parse::<u64>().unwrap(),
            });
            self.transaction.insert(&token_id, &trans);
            result = true;
         } else {
            self.transaction.insert(&token_id, &TransactionJson {
                token_id: token_id.clone(),
                operations: 1,
                sales: sales,
                operation_history: vec![OperationHistory {
                    owner_id: owner_id.to_string(),
                    price: final_price,
                    date: env::block_timestamp().to_string().parse::<u64>().unwrap(),
                }],
            }); 
            result = true;
         };
         result
    }
    

    #[payable]
    pub fn set_review(
        &mut self, 
        review: String, 
        critics: i8,
        token_id: TokenSeriesId
    ) -> Review {
        let token = token_id.split(TOKEN_DELIMETER).collect::<Vec<&str>>();
        let user_id = env::signer_account_id();

        if token.len() == 2 {
            let mut token_series = self.token_series_by_id.get(&token[0].to_string()).expect("Token series not exist");
            let owner_id = self.tokens.owner_by_id.get(&token_id.clone()).expect("Token not exist");
            assert!(owner_id == user_id.clone(), "You must own a token from this series to be able to leave a review");
            
            let data = Review {
                user_id: user_id.clone(),
                review: review.to_string(),
                critics: critics,
            };

            token_series.reviews.insert(&data);

            self.token_series_by_id.insert(&token[0].to_string(), &token_series);

            env::log(b"Review is created");

            data

        } else {
            env::panic(b"Token id invalid");
        }
    }


    // views

    pub fn get_author_market(&self) -> Vec<AccountId> {
		self.profile.iter().filter(|(k, _v)| self.marketplace.iter().find(|(_k2, s2)| s2.creator_id == k.to_string()).is_some())
        .map(|(k, _v)| k.to_string()).collect::<Vec<String>>()    
	}
    

    pub fn get_nft_series_single(&self, token_series_id: TokenSeriesId) -> TokenSeriesJson {
		let token_series = self.token_series_by_id.get(&token_series_id).expect("Series does not exist");
		TokenSeriesJson{
            token_series_id,
			metadata: token_series.metadata,
			creator_id: token_series.creator_id,
            royalty: token_series.royalty,
		}
	}
    
    pub fn get_market(&self,
        token: Option<TokenSeriesId>,
        owner: Option<AccountId>,
        creator_id: Option<AccountId>,
        category: Option<i128>,
        from_index: Option<U128>,
        limit: Option<u64>) -> Vec<MarketView> {
            
            let start_index: u128 = from_index.map(From::from).unwrap_or_default();
            assert!(
                (self.marketplace.len() as u128) > start_index,
                "Out of bounds, please use a smaller from_index."
            );
            let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
            assert_ne!(limit, 0, "Cannot provide limit of 0.");

        let mut result: Vec<MarketView> = self.marketplace.iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|(k, s)| MarketView {
            token_series_id: k.to_string(),
            metadata: s.metadata,
            owner_id: s.owner_id,
            creator_id: s.creator_id,
            price: s.price,
            category: s.category,
            royalty: s.royalty,
            reviews: self.nft_review(k.to_string()),
        }).collect();

        if token.is_some() {
            let token_id = token.unwrap().clone();
            result = result.iter().filter(|x| x.token_series_id == token_id)
                        .skip(start_index as usize)
                        .take(limit)
                        .map(|x| MarketView {
                        token_series_id: x.token_series_id.to_string(),
                        metadata: x.metadata.clone(),
                        owner_id: x.owner_id.clone(),
                        creator_id: x.creator_id.clone(),
                        price: x.price,
                        category: x.category.clone(),
                        royalty: x.royalty.clone(),
                        reviews: self.nft_review(x.token_series_id.to_string()),
                    }).collect();
        };

        if owner.is_some() {
            let owner_id = owner.unwrap().clone();
            result = result.iter().filter(|x| x.owner_id == owner_id)
                        .skip(start_index as usize)
                        .take(limit)
                        .map(|x| MarketView {
                        token_series_id: x.token_series_id.to_string(),
                        metadata: x.metadata.clone(),
                        owner_id: x.owner_id.clone(),
                        creator_id: x.creator_id.clone(),
                        price: x.price,
                        category: x.category.clone(),
                        royalty: x.royalty.clone(),
                        reviews: self.nft_review(x.token_series_id.to_string()),
                    }).collect();
        };
        //near create-account venix.venixcon.testnet --masterAccount venixcon.testnet --initialBalance 50
        if creator_id.is_some() {
            let creator = creator_id.unwrap().clone();
            result = result.iter().filter(|x| x.creator_id == creator)
                        .skip(start_index as usize)
                        .take(limit)
                        .map(|x| MarketView {
                        token_series_id: x.token_series_id.to_string(),
                        metadata: x.metadata.clone(),
                        owner_id: x.owner_id.clone(),
                        creator_id: x.creator_id.clone(),
                        price: x.price,
                        category: x.category.clone(),
                        royalty: x.royalty.clone(),
                        reviews: self.nft_review(x.token_series_id.to_string()),
                    }).collect();
        };

        if category.is_some() {
            result = result.iter().filter(|x| x.category.get(&category.unwrap()).is_some())
                        .skip(start_index as usize)
                        .take(limit)
                        .map(|x| MarketView {
                        token_series_id: x.token_series_id.to_string(),
                        metadata: x.metadata.clone(),
                        owner_id: x.owner_id.clone(),
                        creator_id: x.creator_id.clone(),
                        price: x.price,
                        category: x.category.clone(),
                        royalty: x.royalty.clone(),
                        reviews: self.nft_review(x.token_series_id.to_string()),
                    }).collect();
        };
        
        result
    }

    pub fn get_market_single(&self, token_series_id: TokenSeriesId) -> MarketView {
            
        let data = self.marketplace.get(&token_series_id).expect("Token not exist");
        
        MarketView {
            token_series_id: data.token_series_id.to_string(),
            metadata: data.metadata,
            owner_id: data.owner_id,
            creator_id: data.creator_id,
            price: data.price,
            category: data.category,
            royalty: data.royalty,
            reviews: self.nft_review(data.token_series_id.to_string()),
        }
    }

    

    pub fn get_market_category(&self, category: i128,
        from_index: Option<U128>,
        limit: Option<u64>) -> Vec<MarketView> {
            
            let start_index: u128 = from_index.map(From::from).unwrap_or_default();
            assert!(
                (self.marketplace.len() as u128) > start_index,
                "Out of bounds, please use a smaller from_index."
            );
            let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
            assert_ne!(limit, 0, "Cannot provide limit of 0.");

            self.marketplace.iter().filter(|(_k, s)| s.category.get(&category).is_some())
            .skip(start_index as usize)
            .take(limit)
            .map(|(k, s)| MarketView {
            token_series_id: k.to_string(),
            metadata: s.metadata,
            owner_id: s.owner_id,
            creator_id: s.creator_id,
            price: s.price,
            category: s.category,
            royalty: s.royalty,
            reviews: self.nft_review(k.to_string()),
        }).collect()
    }


    pub fn nft_review(&self, token_id: TokenId) -> Vec<Review> {
        let mut token: String = "".to_string();
        match token_id.split(TOKEN_DELIMETER).collect::<Vec<&str>>().len() {
            1=> {
                    token = token_id;
                },
            2=> {
                    token = token_id.split(TOKEN_DELIMETER).collect::<Vec<&str>>()[0].to_string();
                },
            _=> env::panic(b"token_series_id invalid"),
        };

        let mut token_series = self.token_series_by_id.get(&token).expect("Token series not exist");

        token_series.reviews.iter()
        .map(|x| Review {
            user_id: x.user_id.to_string(),
            review: x.review.to_string(),
            critics: x.critics,
        }).collect()
    }


    pub fn get_Best_sellers(&self) -> Vec<ProfileJson> {
        self.profile.iter().filter(|(_k, s)| s.sales > 0)
        .map(|(k, s)| ProfileJson {
            user_id: k.to_string(),
            name: s.name.unwrap(),
            last_name: s.last_name.unwrap(),
            pen_name: s.pen_name.unwrap(),
            bio: s.bio.unwrap(),
            website: s.website.unwrap(),
            twitter: s.twitter.unwrap(),
            sales: s.sales,
            avatar: s.avatar.unwrap(),
        }).collect()
    }


    pub fn get_nft_series(
        &self,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<TokenSeriesJson> {
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        assert!(
            (self.token_series_by_id.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");

        self.token_series_by_id
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|(token_series_id, token_series)| TokenSeriesJson{
                token_series_id,
                metadata: token_series.metadata,
                creator_id: token_series.creator_id,
                royalty: token_series.royalty,
            })
            .collect()
    }

    pub fn get_nft_series_copy(
        &self,
        token_series_id: TokenSeriesId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token> {
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        let tokens = self.token_series_by_id.get(&token_series_id).unwrap().tokens;
        assert!(
            (tokens.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");

        tokens
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|token_id| self.nft_token(token_id).unwrap())
            .collect()
    }


    pub fn get_nft_series_creator(
        &self,
        creator_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<TokenSeriesJson> {
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        assert!(
            (self.token_series_by_id.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");

        self.token_series_by_id
            .iter().filter(|(_k, s)| s.creator_id == creator_id)
            .skip(start_index as usize)
            .take(limit)
            .map(|(token_series_id, token_series)| TokenSeriesJson{
                token_series_id,
                metadata: token_series.metadata,
                creator_id: token_series.creator_id,
                royalty: token_series.royalty,
            })
            .collect()
    }

    pub fn get_nft_series_category(
        &self,
        creator_id: AccountId,
        category: i128,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<TokenSeriesJson> {
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        assert!(
            (self.token_series_by_id.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");

        self.token_series_by_id
            .iter().filter(|(_k, s)| s.category.get(&category).is_some())
            .skip(start_index as usize)
            .take(limit)
            .map(|(token_series_id, token_series)| TokenSeriesJson{
                token_series_id,
                metadata: token_series.metadata,
                creator_id: token_series.creator_id,
                royalty: token_series.royalty,
            })
            .collect()
    }

    pub fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        let owner_id = self.tokens.owner_by_id.get(&token_id)?;
        let approved_account_ids = self
            .tokens
            .approvals_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id).or_else(|| Some(HashMap::new())));

        let mut token_id_iter = token_id.split(TOKEN_DELIMETER);
        let token_series_id = token_id_iter.next().unwrap().parse().unwrap();
        let series_metadata = self.token_series_by_id.get(&token_series_id).unwrap().metadata;

        let mut token_metadata = self.tokens.token_metadata_by_id.as_ref().unwrap().get(&token_id).unwrap();

        token_metadata.title = Some(format!(
            "{}{}{}",
            series_metadata.title.unwrap(),
            TITLE_DELIMETER,
            token_id_iter.next().unwrap()
        ));

        token_metadata.reference = series_metadata.reference;
        token_metadata.media = series_metadata.media;
        token_metadata.copies = series_metadata.copies;

        Some(Token {
            token_id,
            owner_id,
            metadata: Some(token_metadata),
            approved_account_ids,
        })
    }

}

/*near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);*/
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

fn refund_deposit(storage_used: u64, extra_spend: Balance) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit() - extra_spend;

    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    let refund = attached_deposit - required_cost;
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}



#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::*;

    const MINT_STORAGE_COST: u128 = 5870000000000000000000;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn sample_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("Olympus Mons".into()),
            description: Some("The tallest mountain in the charted solar system".into()),
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.nft_token("1".to_string()), None);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_mint() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());

        let token_id = "0".to_string();
        let token = contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());
        assert_eq!(token.token_id, token_id);
        assert_eq!(token.owner_id, accounts(0).to_string());
        assert_eq!(token.metadata.unwrap(), sample_token_metadata());
        assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_transfer(accounts(1), token_id.clone(), None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        if let Some(token) = contract.nft_token(token_id.clone()) {
            assert_eq!(token.token_id, token_id);
            assert_eq!(token.owner_id, accounts(1).to_string());
            assert_eq!(token.metadata.unwrap(), sample_token_metadata());
            assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
        } else {
            panic!("token not correctly created, or not found by nft_token");
        }
    }

    #[test]
    fn test_approve() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(contract.nft_is_approved(token_id.clone(), accounts(1), Some(1)));
    }

    #[test]
    fn test_revoke() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        // alice revokes bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_revoke(token_id.clone(), accounts(1));
        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(1), None));
    }

    #[test]
    fn test_revoke_all() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build());
        let token_id = "0".to_string();
        contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token_id.clone(), accounts(1), None);

        // alice revokes bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_revoke_all(token_id.clone());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(1), Some(1)));
    }
}
