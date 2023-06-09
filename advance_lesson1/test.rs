```rust
#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get};
use frame_system::ensure_signed;
use sp_std::prelude::*;



#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;


/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// 配置存证vec的长度上限
    type ClaimLength: Get<usize>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    // ---------------------------------
    trait Store for Module<T: Trait> as TemplateModule {
        // Vec<u8> 存证文件的hash值
        Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        ClaimCreated(AccountId, Vec<u8>),
        ClaimRevoked(AccountId, Vec<u8>),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        ProofAlreadyExist,
        CalimNotExist,
        NotClaimOwner,
        ClaimLengthTooLarge
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        // origin 交易的发送方
        // claim 交易文件的哈希值
        #[weight = 0]
        pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            // 校验交易的发送方是签名的，获取交易发送方的accountId sender
            let sender = ensure_signed(origin)?;

            // 检测交易存证的长度过大
            // 保证插入的存证的数据长度小于或者等于ClaimLength
            ensure!(claim.len() <= T::ClaimLength::get() , Error::<T>::ClaimLengthTooLarge);

            // 校验不存在
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            Proofs::<T>::insert(&claim, (sender.clone(), frame_system::Module::<T>::block_number())); //区块数

            Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

            Ok(())
        }

        // origin 交易的发送方
        // claim 交易文件的哈希值
        #[weight = 0]
        pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            // 检测交易存证的长度过大
            // ensure!(claim.len() <= T::ClaimLength::get() , Error::<T>::ClaimLengthTooLarge);

            // 检测文件是否存在
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::CalimNotExist);

            let (owner, _block_number) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

            Ok(())
        }

        // 转移存证文件
        #[weight = 0]
        pub fn transfer_claim(origin, claim: Vec<u8>, dest: T::AccountId) ->dispatch::DispatchResult {
        	let sender = ensure_signed(origin)?;
        	// 检测交易存证的长度过大
            // ensure!(claim.len() <= T::ClaimLength::get() , Error::<T>::ClaimLengthTooLarge);

            // 检测存证文件是否存在
        	ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::CalimNotExist);

            let (owner, _block_number) = Proofs::<T>::get(&claim);

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::insert(&claim, (dest, frame_system::Module::<T>::block_number()));

            Ok(())
        }
    }
}

```

## Test

```rust
use crate::{Module, Trait};
use sp_core::H256;
use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};
use frame_system as system;


// 为测试的test定义了一个Origin表示测试的发送方
impl_outer_origin! {
	pub enum Origin for Test {}
}

// Configure a mock runtime to test the pallet.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64; // AccuntId u64
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

parameter_types! {
	// 设置的存证的长度最大为2
	pub const ClaimLength : usize = 128;
}

impl Trait for Test {
	type Event = (); // 空的元组默认实现了这个event的关联类型的约束
	type ClaimLength = ClaimLength;
}

pub type PoeModule = Module<Test>;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities { // 返回了一个测试用的执行环境
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}


use crate::{Error, mock::*, Trait};
use frame_support::{assert_ok, assert_noop};
use super::*;



// 测试存证创建成功
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];

        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Module::<Test>::block_number()))
    })
}

// 创建存证失败，因为已经有一个同名的存证存在
#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!( // assert_noops! 不会修改链上的状态
            PoeModule::create_claim(Origin::signed(1), claim.clone()), // 断言生成的是error
            Error::<Test>::ProofAlreadyExist
        );
    })
}

// 测试吊销存证成功
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
    })
}

// 吊销存证但是存证不存在
#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::CalimNotExist
        );
    })
}

// 吊销存证但是不是交易的发送方
#[test]
fn revoke_claim_failed_when_is_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];

        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

// 测试转移存证成功
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _  = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 23u64));

        assert_eq!(Proofs::<Test>::get(&claim), (23, frame_system::Module::<Test>::block_number()));

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

// 测试转移存证，但是转移的发起者不是交易的发送方
#[test]
fn transfer_claim_failed_when_is_transfer_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 23),
            Error::<Test>::NotClaimOwner
        );
    })
}

// 测试转移的存证数据不存在
#[test]
fn transfer_claim_failed_when_claim_no_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        let claim_temp = vec![2, 3];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim_temp.clone(), 23),
            Error::<Test>::CalimNotExist
        );
    })
}

#[test]
fn create_claim_failed_when_claim_length_is_too_large() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1, 2];

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimLengthTooLarge,
        );
    })
}