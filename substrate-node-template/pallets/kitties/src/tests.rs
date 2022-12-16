use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(KittiesModule::create_kitty(RuntimeOrigin::signed(ALICE)));
		println!("Hash ALICE:{:?}",KittiesModule::kitty_owned(ALICE) );
		System::set_block_number(2);
		assert_ok!(KittiesModule::create_kitty(RuntimeOrigin::signed(BOB)));
		println!("Hash BOB:{:?}",KittiesModule::kitty_owned(BOB) );
		System::set_block_number(3);
		assert_ok!(KittiesModule::create_kitty(RuntimeOrigin::signed(ALICE)));
		println!("Hash ALICE lan 2:{:?}",KittiesModule::kitty_owned(ALICE) );

		let hash = "4ac59b7cb3f52c068d671e464f0f201350cdc87883e1f16338db6830f511cf3d";
		let h256 = match array_bytes::hex_n_into::<_, H256, 32>(hash) {
			Ok(value) => value,
			Err(e) => panic!("Error:{:?}",e)
			
		};

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(ALICE),EVE,h256 ));

		println!("Hash ALICE transfer:{:?}",KittiesModule::kitty_owned(ALICE) );
		println!("Hash EVE receive:{:?}",KittiesModule::kitty_owned(EVE) );

		let kitty = KittiesModule::get_kitty(h256);

		println!("Kitty dna :{:?}",kitty.clone().unwrap().dna);
		println!("Kitty owner:{:?}",kitty.clone().unwrap().owner);
		println!("Kitty gender:{:?}",kitty.unwrap().gender);

	});

	// block height : thứ tự block
	// block weight: weight là gì ? liên quan transaction -> một cái block sẽ chứa bao nhiêu transaction -> dựa vào maximum block weight
	// set max weight của 1 block : 10 weight
	//giả sử 1 transaction : 2 weight
	// mình cótheer tối đa thực hiện 5 transaction 

	// block size = block length
	// block length : 5MB The maximum length of a block (in bytes).


	// standalone chain , relay, parachain -> substrate 
	// standalone chain -> substrate node template : cũng có dự án 
	// relay: polkadot, kusama, 
	// parachain : polkadotjs để xem 
	// consensus: substrate node template -> private (aura, grandpa)
	// cung cấp cho mình babe,grandpa (POS), POW
	// rococo
	// mock node giống như mock runtime
	


}

