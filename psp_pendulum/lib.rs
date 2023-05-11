// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
#![cfg_attr(not(feature = "std"), no_std)]

use ethnum::U256;
use ink::prelude::vec::Vec;
mod psp_pendulum_lib;

// use crate::pallet_assets::*;
// use brush::{
// 	contracts::psp22::{utils::*, PSP22Error, *},
// 	modifiers,
// };
use crate::psp_pendulum_lib::PSP22Error;

// #[brush::contract]
#[ink::contract]
mod my_psp22_pallet_asset {
    use crate::*;
    use ink::codegen::StaticEnv;

    #[ink(storage)]
    #[derive(Default)]
    pub struct MyPSP22 {
        pub asset_id: (u8, [u8; 12], [u8; 32]),
        pub origin_type: u8,
        pub name: Vec<u8>,
        pub symbol: Vec<u8>,
        pub decimals: u8,
    }

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(
            origin_type: psp_pendulum_lib::OriginType,
            asset_id: (u8, [u8; 12], [u8; 32]),
            name: Vec<u8>,
            symbol: Vec<u8>,
            decimals: u8,
        ) -> Self {
            let origin_type = if origin_type == psp_pendulum_lib::OriginType::Caller {
                0
            } else {
                1
            };
            Self {
                origin_type,
                asset_id,
                name,
                symbol,
                decimals,
            }
        }

        #[ink(message)]
        pub fn get_address(&self) -> [u8; 32] {
            let caller = self.env().caller();
            *caller.as_ref()
        }
        
        #[ink(message, selector = 0x06fdde03)]
        pub fn name(&self) -> Vec<u8> {
            self.name.clone()
        }

        #[ink(message, selector = 0x95d89b41)]
        pub fn symbol(&self) -> Vec<u8> {
            self.symbol.clone()
        }

        #[ink(message, selector = 0x313ce567)]
        pub fn decimals(&self) -> u8 {
            self.decimals.clone()
        }

        #[ink(message, selector = 0x18160ddd)]
        pub fn total_supply(&self) -> [u128; 2] {
            let b = self._total_supply();
            let total_supply_u256: U256 = U256::try_from(b).unwrap();
            total_supply_u256.0
        }

        #[ink(message, selector = 0x70a08231)]
        pub fn balance_of(&self, account: AccountId) -> [u128; 2] {
            let b = self._balance_of(account);
            let balance_u256: U256 = U256::try_from(b).unwrap();
            balance_u256.0
        }

        #[ink(message, selector = 0xa9059cbb)]
        pub fn transfer(&mut self, to: AccountId, amount: [u128; 2]) {
            let amount: u128 = U256(amount).try_into().unwrap();
            self._transfer(to, amount, Vec::<u8>::new())
                .expect("should transfer");
        }

        #[ink(message, selector = 0x23b872dd)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: [u128; 2]) {
            let amount: u128 = U256(amount).try_into().unwrap();
            self._transfer_from(from, to, amount, Vec::<u8>::new())
                .expect("should transfer from");
        }

        #[ink(message, selector = 0x095ea7b3)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) {
            self._approve(spender, value).unwrap();
        }

        #[ink(message, selector = 0xdd62ed3e)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> [u128; 2] {
            let b = self._allowance(owner, spender);
            let balance_u256: U256 = U256::try_from(b).unwrap();
            balance_u256.0
        }
    }

    impl MyPSP22 {
        fn _total_supply(&self) -> Balance {
            psp_pendulum_lib::PendulumChainExt::total_supply(self.asset_id).unwrap()
        }

        fn _balance_of(&self, owner: AccountId) -> Balance {
            psp_pendulum_lib::PendulumChainExt::balance(self.asset_id, *owner.as_ref()).unwrap()
        }

        fn _transfer(
            &mut self,
            to: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let origin: psp_pendulum_lib::OriginType = self.origin_type.into();
            let result = psp_pendulum_lib::PendulumChainExt::transfer(
                origin,
                self.asset_id,
                *to.as_ref(),
                value.into(),
            );
            match result {
                Result::<(), psp_pendulum_lib::PalletAssetErr>::Ok(_) => {
                    Result::<(), PSP22Error>::Ok(())
                }
                Result::<(), psp_pendulum_lib::PalletAssetErr>::Err(e) => {
                    Result::<(), PSP22Error>::Err(PSP22Error::from(e))
                }
            }
        }

        fn _transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let origin: psp_pendulum_lib::OriginType = self.origin_type.into();
            let transfer_approved_result = psp_pendulum_lib::PendulumChainExt::transfer_approved(
                origin,
                self.asset_id,
                *from.as_ref(),
                *to.as_ref(),
                value.into(),
            );
            match transfer_approved_result {
                Result::<(), psp_pendulum_lib::PalletAssetErr>::Ok(_) => {
                    Result::<(), PSP22Error>::Ok(())
                }
                Result::<(), psp_pendulum_lib::PalletAssetErr>::Err(e) => {
                    Result::<(), PSP22Error>::Err(PSP22Error::from(e))
                }
            }
        }

        fn _approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
            let origin: psp_pendulum_lib::OriginType = self.origin_type.into();
            let approve_transfer_result = psp_pendulum_lib::PendulumChainExt::approve_transfer(
                origin,
                self.asset_id,
                *spender.as_ref(),
                value.into(),
            );
            match approve_transfer_result {
                Result::<(), psp_pendulum_lib::PalletAssetErr>::Ok(_) => {
                    Result::<(), PSP22Error>::Ok(())
                }
                Result::<(), psp_pendulum_lib::PalletAssetErr>::Err(e) => {
                    Result::<(), PSP22Error>::Err(PSP22Error::from(e))
                }
            }
        }

        fn _allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            psp_pendulum_lib::PendulumChainExt::allowance(
                self.asset_id,
                *owner.as_ref(),
                *spender.as_ref(),
            )
            .unwrap()
        }
    }
}
