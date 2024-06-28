use crate as pallet_weights;
use crate::mock::*;
use frame_support::weights::{ RuntimeDbWeight, Weight };
use frame_support::dispatch::GetDispatchInfo;
use sp_core::Get;

#[test]
fn verify_address_test() {
	new_test_ext().execute_with(|| {
		let db_weights: RuntimeDbWeight = <TestRuntime as frame_system::Config>::DbWeight::get();
		let weight = pallet_weights::Call::<TestRuntime>::verify_address {}.get_dispatch_info().weight;
		assert_eq!(weight, Weight::from_parts(10_000, u64::MAX) + DbWeight::get().reads(1));
	});
}

#[test]
fn duplicate_test() {
	new_test_ext().execute_with(|| {
		let db_weights: RuntimeDbWeight = <TestRuntime as frame_system::Config>::DbWeight::get();
		let weight1 = pallet_weights::Call::<TestRuntime>::duplicate_and_store { elem: 0, count: 1 }
			.get_dispatch_info()
			.weight;
		let weight2 = pallet_weights::Call::<TestRuntime>::duplicate_and_store {
			elem: 0,
			count: 1000,
		}
		.get_dispatch_info()
		.weight;

		assert!(weight1.proof_size() < weight2.proof_size());
		assert!(weight1.proof_size() > db_weights.writes(1).proof_size());
	});
}

#[test]
fn store_maybe_hashed_test() {
	new_test_ext().execute_with(|| {
		let weight1 = pallet_weights::Call::<TestRuntime>::store_maybe_hashed {
			data: vec![1, 2, 3],
			hash: true,
		}
		.get_dispatch_info()
		.weight;
		let weight2 = pallet_weights::Call::<TestRuntime>::store_maybe_hashed {
			data: vec![1, 2, 3],
			hash: false,
		}
		.get_dispatch_info()
		.weight;

		assert_eq!(weight1, Weight::from_parts(100_000, u64::MAX));
		assert_eq!(weight2, Weight::from_parts(10_000, u64::MAX));
	});
}

#[test]
fn benchmarked_store_maybe_hashed_test() {
	new_test_ext().execute_with(|| {
		let long_vec = vec![1; 100000];
		let weight1 = pallet_weights::Call::<TestRuntime>::benchmarked_store_maybe_hashed {
			data: long_vec.clone(),
			hash: true,
		}
		.get_dispatch_info()
		.weight;
		let weight2 = pallet_weights::Call::<TestRuntime>::benchmarked_store_maybe_hashed {
			data: long_vec,
			hash: false,
		}
		.get_dispatch_info()
		.weight;

		assert!(weight1.proof_size() > weight2.proof_size());
	});
}
