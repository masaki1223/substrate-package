// initialise with:
// post({sender: runtime.indices.ss58Decode('F7Gh'), call: calls.demo.setPayment(1000)}).tie(console.log)

use parity_codec::Encode;
use support::{StorageValue, dispatch::Result, decl_module, decl_storage};
use runtime_primitives::traits::{Zero, Hash, CheckedAdd, CheckedSub};
use system::ensure_signed;

pub trait Trait: balances::Trait {}

decl_storage! {
	trait Store for Module<T: Trait> as Demo {
		Payment get(payment): Option<T::Balance>;
		Pot get(pot): T::Balance;
		Nonce get(nonce): u64;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn play(origin) -> Result {
			let sender = ensure_signed(origin)?;
			let payment = Self::payment().ok_or("Must have payment amount set")?;
			
			let mut nonce = Self::nonce();
			let mut pot = Self::pot();
			let mut sender_free_balance = <balances::Module<T>>::free_balance(&sender);
			sender_free_balance = sender_free_balance.checked_sub(&payment).ok_or("User does not have enough funds to play the game")?;

			if (<system::Module<T>>::random_seed(), &sender, nonce)
				.using_encoded(<T as system::Trait>::Hashing::hash)
				.using_encoded(|e| e[0] < 128)
			{
				sender_free_balance = sender_free_balance.checked_add(&pot).ok_or("Overflow when adding funds to user account")?;
				pot = Zero::zero();
			}

			pot = pot.checked_add(&payment).ok_or("Overflow when adding funds to pot")?;
			nonce = nonce.wrapping_add(1);

			<balances::Module<T>>::set_free_balance(&sender, sender_free_balance);
			<Pot<T>>::put(pot);
			<Nonce<T>>::put(nonce);

			Ok(())
		}

		fn set_payment(origin, value: T::Balance) -> Result {
			let _sender = ensure_signed(origin)?;

			if Self::payment().is_none() {
				<Payment<T>>::put(value);
				<Pot<T>>::put(value);
			}

			Ok(())
		}
	}
}
