use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

// 使用cargo test  进行全部测试文件的测试

// test标签表示是一个测试用例
#[test]
fn claim_should_work() {
    // 构建一个测试环境
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];
        let limit = StringLimit::get() as usize;

        // 设置账户是1（AccountId是u64，可以为1）， 将claim传入
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        // 验证这个区块是值的，第一个值是账户1，第二个是区块高度
        assert_eq!(
            Proofs::<Test>::get(&claim), 
            (1, frame_system::Pallet::<Test>::block_number())
        );
        // 验证超过limit长度的cliam在创建时候的报错
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), vec![0u8; limit + 1]),
            Error::<Test>::BadMetadata
        );
    })
}

#[test]
fn claim_failed_when_claim_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];
        // 同理了
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        // 判断是否已存在，存在时创建会报错
        assert_noop!(
            PoeModule::create_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::ProofAlreadyClaimed
        );
    })
}

#[test]
fn revoke_should_work() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];
        // 先创建
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        // 再销毁
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        // 期望：此时已经不存在了
        assert_eq!(Proofs::<Test>::contains_key(&claim), false);
    })
}

#[test]
fn revoke_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];
        // 创建存证
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        // 原账户是1，现在请求发送方是2，不能销毁存证，期望是销毁时候返回false
        assert_noop!(PoeModule::revoke_claim(Origin::signed(2), claim.clone()), Error::<Test>::NotProofOwner);
    })
}

#[test]
fn revoke_failed_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];
        // 期望：不存在改存证时候返回不存在存证错误
        assert_noop!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()), Error::<Test>::NoSuchProof);
    })
}

#[test]
fn transfer_should_work() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        // 转移存证
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
        // 验证转移情况
        assert_eq!(Proofs::<Test>::get(&claim), (2, frame_system::Pallet::<Test>::block_number()));
    })
}

#[test]
fn transfer_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];
        // 创建存证
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        // 不是该owner时候无法转移
        assert_noop!(PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 1), Error::<Test>::NotProofOwner);
    })
}

#[test]
fn transfer_failed_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];
        // 不存在时候无法转移
        assert_noop!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2), Error::<Test>::NoSuchProof);
    })
}

