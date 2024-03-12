#![cfg_attr(not(feature = "export-abi"), no_main, no_std)]
extern crate alloc;
extern crate mini_alloc;

use crate::erc721a::{ERC721Params, ERC721};
use alloc::{format, string::String, vec::Vec};
use mini_alloc::MiniAlloc;
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: MiniAlloc = MiniAlloc::INIT;

mod erc721a;

pub struct SampleParams;

/// Immutable definitions
impl ERC721Params for SampleParams {
    const NAME: &'static str = "Sample ERC721";
    const SYMBOL: &'static str = "SAMPLE";

    fn token_uri(token_id: U256) -> String {
        format!(
            "ipfs://QmZcH4YvBVVRJtdn4RdbaqgspFU8gH6P9vomDpBVpAL3u4/{}",
            token_id
        )
    }
}

// The contract
sol_storage! {
    #[entrypoint] // Makes SampleNFT the entrypoint
    pub struct SampleNFT {
        #[borrow] // Allows erc721 to access SampleNFT's storage and make calls
        ERC721<SampleParams> erc721a;
    }
}

// Rust implementation of this SampleNFT Solidity contract:

// pragma solidity ^0.8.21;
// import "erc721a/contracts/ERC721A.sol";

// contract SampleNFT is ERC721A {
//     constructor() ERC721A("Sample NFT", "SAMPLE") {}

//     function mint(uint256 qty) external {
//         _mint(msg.sender, qty);
//     }

//     function burn(uint256 tokenId) external {
//         _burn(tokenId, true);
//     }
// }

#[external]
#[inherit(ERC721<SampleParams>)]
impl SampleNFT {
    pub fn mint(&mut self, qty: U256) -> Result<(), Vec<u8>> {
        self.erc721a._mint(msg::sender(), qty)?;
        Ok(())
    }

    pub fn burn(&mut self, token_id: U256) -> Result<(), Vec<u8>> {
        self.erc721a._burn(token_id, true)?;
        Ok(())
    }
}
