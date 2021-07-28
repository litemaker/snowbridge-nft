//! BasicInboundChannel pallet benchmarking

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_system::{RawOrigin, self, EventRecord};
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
use hex_literal::hex;
use sp_std::convert::TryInto;

use snowbridge_core::{ChannelId, Message, MessageId, Proof};
use snowbridge_ethereum::{Log, Header};

#[allow(unused_imports)]
use crate::inbound::Module as BasicInboundChannel;

fn assert_last_event<T: Config>(system_event: <T as frame_system::Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

// This collection of benchmarks should include a benchmark for each
// call dispatched by the channel, i.e. each "app" pallet function
// that can be invoked by MessageDispatch. The most expensive call
// should be used in the `submit` benchmark.
//
// We rely on configuration via chain spec of the app pallets because
// we don't have access to their storage here.
benchmarks! {
	// Benchmark `submit` extrinsic under worst case conditions:
	// * `submit` dispatches the DotApp::unlock call
	// * `unlock` call successfully unlocks DOT
	submit {
		let caller: T::AccountId = whitelisted_caller();
		let (header, message) = dot_unlock_data();
		let envelope: envelope::Envelope = rlp::decode::<Log>(&message.data)
			.map(|log| log.try_into().unwrap())
			.unwrap();
		Nonce::put(envelope.nonce - 1);
		SourceChannel::put(envelope.channel);

		T::Verifier::initialize_storage(
			vec![header],
			0.into(),
			0, // forces all headers to be finalized
		)?;

	}: _(RawOrigin::Signed(caller.clone()), message)
	verify {
		assert_eq!(envelope.nonce, Nonce::get());

		let message_id = MessageId::new(ChannelId::Basic, envelope.nonce);
		if let Some(event) = T::MessageDispatch::successful_dispatch_event(message_id) {
			assert_last_event::<T>(event);
		}
	}

	#[extra]
	submit_eth_mint {
		let caller: T::AccountId = whitelisted_caller();
		let (header, message) = eth_mint_data();
		let envelope: envelope::Envelope = rlp::decode::<Log>(&message.data)
			.map(|log| log.try_into().unwrap())
			.unwrap();
		Nonce::put(envelope.nonce - 1);
		SourceChannel::put(envelope.channel);

		T::Verifier::initialize_storage(
			vec![header],
			0.into(),
			0, // forces all headers to be finalized
		)?;

	}: submit(RawOrigin::Signed(caller.clone()), message)
	verify {
		assert_eq!(envelope.nonce, Nonce::get());

		let message_id = MessageId::new(ChannelId::Basic, envelope.nonce);
		if let Some(event) = T::MessageDispatch::successful_dispatch_event(message_id) {
			assert_last_event::<T>(event);
		}
	}

	#[extra]
	submit_erc20_mint {
		let caller: T::AccountId = whitelisted_caller();
		let (header, message) = erc20_mint_data();
		let envelope: envelope::Envelope = rlp::decode::<Log>(&message.data)
			.map(|log| log.try_into().unwrap())
			.unwrap();
		Nonce::put(envelope.nonce - 1);
		SourceChannel::put(envelope.channel);

		T::Verifier::initialize_storage(
			vec![header],
			0.into(),
			0, // forces all headers to be finalized
		)?;

	}: submit(RawOrigin::Signed(caller.clone()), message)
	verify {
		assert_eq!(envelope.nonce, Nonce::get());

		let message_id = MessageId::new(ChannelId::Basic, envelope.nonce);
		if let Some(event) = T::MessageDispatch::successful_dispatch_event(message_id) {
			assert_last_event::<T>(event);
		}
	}
}

// ETH mint
// Channel = 0x2ffa5ecdbe006d30397c7636d3e015eee251369f
// Nonce = 3
// Source = 0x774667629726ec1fabebcec0d9139bd1c8f72a23
fn eth_mint_data() -> (Header, Message) {
	(
		Header {
			parent_hash: hex!("0db25049c0e7fe65be6d70d8901a1d285b403013063dab4462dc09da4c114729").into(),
			timestamp: 1619679041u64.into(),
			number: 92u64.into(),
			author: hex!("0000000000000000000000000000000000000000").into(),
			transactions_root: hex!("82ac39ca8d4fef5f60db628aabf1273fca954afd36205ff7fbf48a9c12ad4ea4").into(),
			ommers_hash: hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347").into(),
			extra_data: hex!("").into(),
			state_root: hex!("5430db6edfe87449f6082bf6804815b60b78636faa076e3f56c977eec519f012").into(),
			receipts_root: hex!("ad6f0b524225ce38a94f27a411633d9acd647e3905aae783cd2f3e82b2035f77").into(),
			logs_bloom: (&hex!("00000008000000000000000000000000000000000000400000000000010000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000008000000000000000000000000000000000000020000000000000000000000000080000000000000004000000020000000000000000000000000000000000000800002000000000000000000000000000000000000000000000000000000000000000")).into(),
			gas_used: 63853u64.into(),
			gas_limit: 6721975u64.into(),
			difficulty: 0u64.into(),
			seal: vec![
				hex!("a00000000000000000000000000000000000000000000000000000000000000000").to_vec(),
				hex!("880000000000000000").to_vec(),
			],
			base_fee: None,
		},
		Message {
			data: hex!("f90119942ffa5ecdbe006d30397c7636d3e015eee251369fe1a0779b38144a38cfc4351816442048b17fe24ba2b0e0c63446b576e8281160b15bb8e0000000000000000000000000774667629726ec1fabebcec0d9139bd1c8f72a23000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000057410189b4ab1ef20763630df9743acf155865600daff200d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0000c16ff2862300000000000000000000000000000000000000000000000000000000000000000000").to_vec(),
			proof: Proof {
				block_hash: hex!("9cd97b10d2087810a0ebeab0f5dcec166a50ab6923a1b21e64f65f4c6deee65d").into(),
				tx_index: 0,
				data: (
					vec![hex!("ad6f0b524225ce38a94f27a411633d9acd647e3905aae783cd2f3e82b2035f77").to_vec()],
					vec![hex!("f902ca822080b902c4f902c10182f96db9010000000008000000000000000000000000000000000000400000000000010000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000008000000000000000000000000000000000000020000000000000000000000000080000000000000004000000020000000000000000000000000000000000000800002000000000000000000000000000000000000000000000000000000000000000f901b7f89994774667629726ec1fabebcec0d9139bd1c8f72a23e1a0caae0f5e72020d428da73a237d1f9bf162e158dda6d4908769b8b60c095b01f4b86000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d000000000000000000000000000000000000000000000000002386f26fc10000f90119942ffa5ecdbe006d30397c7636d3e015eee251369fe1a0779b38144a38cfc4351816442048b17fe24ba2b0e0c63446b576e8281160b15bb8e0000000000000000000000000774667629726ec1fabebcec0d9139bd1c8f72a23000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000057410189b4ab1ef20763630df9743acf155865600daff200d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0000c16ff2862300000000000000000000000000000000000000000000000000000000000000000000").to_vec()],
				),
			},
		},
	)
}

// ERC20 mint
// Channel = 0x2ffa5ecdbe006d30397c7636d3e015eee251369f
// Nonce = 2
// Source = 0x83428c7db9815f482a39a1715684dcf755021997
fn erc20_mint_data() -> (Header, Message) {
	(
		Header {
			parent_hash: hex!("2c89d0bdd9bd57611ca10ad3a765a8c3698c60c00f2f004b142454ed46d539a4").into(),
			timestamp: 1619678951u64.into(),
			number: 77u64.into(),
			author: hex!("0000000000000000000000000000000000000000").into(),
			transactions_root: hex!("99fd65851a4706d0ca4bdda6638fdb73276643aa440397bef424ab56b77f729d").into(),
			ommers_hash: hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347").into(),
			extra_data: hex!("").into(),
			state_root: hex!("a97664779a82ed077bb0a57f3b754c6c0a46124c6af172447ba2b67a08d91647").into(),
			receipts_root: hex!("3f7a9cb9d4b9db18280e882d1ddfef469bcb1097b2d8b1f9f9a0b2abeebbae7a").into(),
			logs_bloom: (&hex!("00000008000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000408202000000000000000000000000000000000000000000000000000000000000000000000200010000000000000000010001000000000000008000000000000000000000000100000000000000000840000008000020000000000000001000000000000000000000000000001000000000200000000000002000004000000020000000000000000000008000000800000280c00000010000000000000000020000000000220000000000000000000000000000000")).into(),
			gas_used: 230365u64.into(),
			gas_limit: 6721975u64.into(),
			difficulty: 0u64.into(),
			seal: vec![
				hex!("a00000000000000000000000000000000000000000000000000000000000000000").to_vec(),
				hex!("880000000000000000").to_vec(),
			],
			base_fee: None,
		},
		Message {
			data: hex!("f9013a942ffa5ecdbe006d30397c7636d3e015eee251369fe1a0779b38144a38cfc4351816442048b17fe24ba2b0e0c63446b576e8281160b15bb9010000000000000000000000000083428c7db9815f482a39a1715684dcf75502199700000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000006b4201f8f7758fbcefd546eaeff7de24aff666b6228e7389b4ab1ef20763630df9743acf155865600daff200d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27de803000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").to_vec(),
			proof: Proof {
				block_hash: hex!("9db154065a223a7d3a7124cb3fbf8214f8c7715aaa97d2865a5f95dd58344df6").into(),
				tx_index: 0,
				data: (
					vec![
						hex!("3f7a9cb9d4b9db18280e882d1ddfef469bcb1097b2d8b1f9f9a0b2abeebbae7a").to_vec(),
						hex!("f08f9dfb061a36bcc994292013e9c73d5a1e380dfad010a3450029af352ff670").to_vec(),
					],
					vec![
						hex!("f851a08e9357a1f77e895cecddf86c2a17aa1027f04aab509e0ae1be640d8190f3704780808080808080a0f08f9dfb061a36bcc994292013e9c73d5a1e380dfad010a3450029af352ff6708080808080808080").to_vec(),
						hex!("f9044430b90440f9043d0183016555b9010000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000008002000000000000000000000000000000000000000000000000000000000000000000000200010000000000000000010001000000000000008000000000000000000000000100000000000000000840000008000020000000000000001000000000000000000000000000001000000000200000000000002000004000000020000000000000000000008000000800000200c00000010000000000000000000000000000020000000000000000000000000000000f90332f89b94f8f7758fbcefd546eaeff7de24aff666b6228e73f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2a000000000000000000000000083428c7db9815f482a39a1715684dcf755021997a000000000000000000000000000000000000000000000000000000000000003e8f89b94f8f7758fbcefd546eaeff7de24aff666b6228e73f863a08c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925a000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2a000000000000000000000000083428c7db9815f482a39a1715684dcf755021997a00000000000000000000000000000000000000000000000000000000000000000f8b99483428c7db9815f482a39a1715684dcf755021997e1a01e7b27577112ed83d53de87b38aee59ab80d8a9ba4acd90aad6cfee917534c79b880000000000000000000000000f8f7758fbcefd546eaeff7de24aff666b6228e7300000000000000000000000089b4ab1ef20763630df9743acf155865600daff2d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d00000000000000000000000000000000000000000000000000000000000003e8f9013a942ffa5ecdbe006d30397c7636d3e015eee251369fe1a0779b38144a38cfc4351816442048b17fe24ba2b0e0c63446b576e8281160b15bb9010000000000000000000000000083428c7db9815f482a39a1715684dcf75502199700000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000006b4201f8f7758fbcefd546eaeff7de24aff666b6228e7389b4ab1ef20763630df9743acf155865600daff200d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27de803000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").to_vec(),
					],
				),
			},
		},
	)
}

// DOT unlock
// Channel = 0x2ffa5ecdbe006d30397c7636d3e015eee251369f
// Nonce = 1
// Source = 0xb1185ede04202fe62d38f5db72f71e38ff3e8305
fn dot_unlock_data() -> (Header, Message) {
	(
		Header {
			parent_hash: hex!("07966c0314890b0a506ac5e29c934a1a7d77245ce088fb5786eec3a6f1b855c8").into(),
			timestamp: 1619678891u64.into(),
			number: 67u64.into(),
			author: hex!("0000000000000000000000000000000000000000").into(),
			transactions_root: hex!("979b37112184a16bc05f7a6a12eb0b6bd277c1188741315e92aba3517329b091").into(),
			ommers_hash: hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347").into(),
			extra_data: hex!("").into(),
			state_root: hex!("4e8278dec2498fd15d8d6d46f42204f4907161adceb6f3a6ad046cb3e939d403").into(),
			receipts_root: hex!("d21685ddfe4768946b1a31abe7fba388d90b90245127ced39960061a6b18be9e").into(),
			logs_bloom: (&hex!("00000008000040000000000000000200000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000400000008000000000000000000008000000000000000000000000000020000000000000000000800000001000400000000000010000000000000000000000000000000000400000000100000000000000000040000008000000000000000000000000000000000000000000000000001000000000200000000000002000004000000020000000000000000000000000000000000000820400000000000000000000000000000000000000000000000000000000000000000")).into(),
			gas_used: 83009u64.into(),
			gas_limit: 6721975u64.into(),
			difficulty: 0u64.into(),
			seal: vec![
				hex!("a00000000000000000000000000000000000000000000000000000000000000000").to_vec(),
				hex!("880000000000000000").to_vec(),
			],
			base_fee: None,
		},
		Message {
			data: hex!("f90119942ffa5ecdbe006d30397c7636d3e015eee251369fe1a0779b38144a38cfc4351816442048b17fe24ba2b0e0c63446b576e8281160b15bb8e0000000000000000000000000b1185ede04202fe62d38f5db72f71e38ff3e8305000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000057400189b4ab1ef20763630df9743acf155865600daff200d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d000064a7b3b6e00d000000000000000000000000000000000000000000000000000000000000000000").to_vec(),
			proof: Proof {
				block_hash: hex!("465a2577662aca511a2453d102e2f1452ab2598c28ab1a9b81ad2ca5e6350d78").into(),
				tx_index: 0,
				data: (
					vec![hex!("d21685ddfe4768946b1a31abe7fba388d90b90245127ced39960061a6b18be9e").to_vec()],
					vec![hex!("f9040c822080b90406f904030183014441b9010000000008000040000000000000000200000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000400000008000000000000000000008000000000000000000000000000020000000000000000000800000001000400000000000010000000000000000000000000000000000400000000100000000000000000040000008000000000000000000000000000000000000000000000000001000000000200000000000002000004000000020000000000000000000000000000000000000820400000000000000000000000000000000000000000000000000000000000000000f902f8f9013c94672a95c8928c8450b594186cf7954ec269626a2df863a0a78a9be3a7b862d26933ad85fb11d80ef66b8f972d7cbba06621d583943a4098a0000000000000000000000000b1185ede04202fe62d38f5db72f71e38ff3e8305a000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2b8c00000000000000000000000000000000000000000000000000de0b6b3a7640000000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000020d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0000000000000000000000000000000000000000000000000000000000000000f89b94672a95c8928c8450b594186cf7954ec269626a2df863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000089b4ab1ef20763630df9743acf155865600daff2a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000de0b6b3a7640000f90119942ffa5ecdbe006d30397c7636d3e015eee251369fe1a0779b38144a38cfc4351816442048b17fe24ba2b0e0c63446b576e8281160b15bb8e0000000000000000000000000b1185ede04202fe62d38f5db72f71e38ff3e8305000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000057400189b4ab1ef20763630df9743acf155865600daff200d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d000064a7b3b6e00d000000000000000000000000000000000000000000000000000000000000000000").to_vec()],
				),
			},
		},
	)
}

impl_benchmark_test_suite!(
	BasicInboundChannel,
	crate::inbound::test::new_tester(Default::default()),
	crate::inbound::test::Test,
);
