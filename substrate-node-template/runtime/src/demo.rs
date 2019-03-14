// initialize with:
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
		// This function initializes the `payment` storage item
		// It also populates the pot with an initial value
		fn set_payment(origin, value: T::Balance) -> Result {
			// Ensure that the function call is a signed message (i.e. a transaction)
			let _ = ensure_signed(origin)?;
			
			// If `payment` is not initialized with some value
			if Self::payment().is_none() {
				// Set the value of `payment`
				<Payment<T>>::put(value);
				// Initialize the `pot` with the same value
				<Pot<T>>::put(value);
			}

			// Return Ok(()) when everything happens successfully
			Ok(())
		}
		// This function is allows a user to play our coin-flip game
		fn play(origin) -> Result {
			// Ensure that the function call is a signed message (i.e. a transaction)
			// Additionally, derive the sender address from the signed message
			let sender = ensure_signed(origin)?;
			// Ensure that `payment` storage item has been set
			let payment = Self::payment().ok_or("Must have payment amount set")?;
			// Read our storage values, and place them in memory variables
			let mut nonce = Self::nonce();
			let mut pot = Self::pot();
			// The Balances module can tell us how much `free_balance` a user has
			let mut sender_free_balance = <balances::Module<T>>::free_balance(&sender);
			// Check that the user has enough free balance to play the game
			sender_free_balance = sender_free_balance.checked_sub(&payment).ok_or("User does not have enough funds to play the game")?;

			// Generate a random hash between 0-255 using a csRNG algorithm
			if (<system::Module<T>>::random_seed(), &sender, nonce)
				.using_encoded(<T as system::Trait>::Hashing::hash)
				.using_encoded(|e| e[0] < 128)
			{
				// If the user won the coin flip, try to give them the winnings
				sender_free_balance = sender_free_balance.checked_add(&pot).ok_or("Overflow when adding funds to user account")?;
				// Reduce the pot to zero
				pot = Zero::zero();
			}

			// No matter the outcome, increase the pot by the payment amount
			pot = pot.checked_add(&payment).ok_or("Overflow when adding funds to pot")?;
			// Increment the nonce
			nonce = nonce.wrapping_add(1);

			// Store the updated values for our module once we know nothing can fail
			<balances::Module<T>>::set_free_balance(&sender, sender_free_balance);
			<Pot<T>>::put(pot);
			<Nonce<T>>::put(nonce);

			// Return Ok(()) when everything happens successfully
			Ok(())
		}
	}
}
