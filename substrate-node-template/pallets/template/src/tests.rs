use crate::{mock::*, Error, Event as TemplateEvent};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

fn last_event() -> TemplateEvent<Test> {

	System::events().into_iter().map(|r| r.event).filter_map(|e| if let RuntimeEvent::TemplateModule(inner) = e {Some(inner)} else {None})
	.last().unwrap()

}

#[test]
fn it_works_for_event() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		System::set_block_number(1);
		assert_ok!(TemplateModule::do_something(RuntimeOrigin::signed(1), 42));

		System::assert_last_event(crate::Event::SomethingStored(42,1).into());
		assert_eq!(last_event(),TemplateEvent::SomethingStored(42,1));
		println!("Event:{:?}", last_event());
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}


#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
