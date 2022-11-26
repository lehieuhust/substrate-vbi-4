use std::os::unix::net::UnixListener;

mod step1;
mod step2;
mod step3;
mod step4;
mod step5;
fn main() {
    println!("Hello, world!");
}


#[test]
fn test_step1() {

    let mut balances = step1::BalancesModule::new();

    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    let _ = balances.transfer(1, 2, 50);


    assert_eq!(balances.get_balance(1), 50);

    assert_eq!(balances.get_balance(2), 250);

}



#[test]
fn test_step2() {

    let mut balances = step2::BalancesModule::new();

    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    let _ = balances.transfer(1, 2, 50);


    assert_eq!(balances.get_balance(1), 50);

    assert_eq!(balances.get_balance(2), 250);

}


#[test]
fn test_step3() {

    let mut voting = step3::VotingModule::new();

    voting.vote(1, 0, true);


    assert!(voting.get_vote(1, 0));
}




#[test]
fn test_step3_1(){

    let user_1 = 1;
    let user_2 = 2;

    let user_1_1 = 1u64;

    let mut balances = step2::BalancesModule::new();
    let mut voting = step3::VotingModule::new();


    balances.set_balance(user_1, 100);

    voting.vote(user_1_1, 100, true);
}



#[test]
fn test_step4(){
    let mut balances = step4::BalancesModule::<u32,u32>::new(); 

    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    let _ = balances.transfer(1, 2, 50);


    assert_eq!(balances.get_balance(1), 50);

    assert_eq!(balances.get_balance(2), 250);

}



#[test]
fn test_step5(){



    struct Runtime;

    impl step5::Config for Runtime {
        type AccountId = u32;
        type Balance = u32;
        type VoteIndex = u64;
    
    }

    let user_1 =1;

    let user_2 : <Runtime as step5::Config>::AccountId = 1;
    let mut balances = step5::BalancesModule::<Runtime>::new(); 

    balances.set_balance(user_1, 100);
    balances.set_balance(2, 200);

    let _ = balances.transfer(1, 2, 50);


    assert_eq!(balances.get_balance(1), 50);

    assert_eq!(balances.get_balance(2), 250);

}
