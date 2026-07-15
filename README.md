# Kont

Kont is an ultra-lean, bare-metal, `no_std` SPL Token infrastructure primitive designed for execution-critical Solana pipelines.

By completely bypassing heavy framework dependencies, dynamic heap allocations (`alloc`), and complex serialization frameworks, Kont acts as a pure, header-style memory-mapping stencil. It allows you to read and write SPL Token states at raw execution speed.

## Why Use Kont?

* **OS & Runtime Independence (`no_std`):** Completely decoupled from standard library file systems and networking. Run the exact same codebase on-chain, inside SGX/Nitro secure enclaves, or flashed directly onto low-power embedded microcontrollers.
* **Minimal Compute Unit (CU) Impact:** Zero heavy parsing loops. Read balances, verify owners, and check mint authorities in single-digit clock cycles.
* **Stack-Allocated Blueprints (Zero Heap):** Construct, parameterize, and export binary-accurate instruction payloads using fixed stack-allocated templates without ever triggering a heap allocation.
* **Safe Zero-Copy Views:** Overlay structured, read-only byte views directly onto raw hardware addresses or RPC byte slices.

---

## Concrete Usage Examples

The following examples demonstrate how to use Kont completely independent of Solana's runtime library wrappers.

### Example 1: Fast Account Balance Inspection (Zero-Copy Read)
Overlaying a memory map directly onto a raw byte buffer (e.g., from an RPC network response or on-chain memory slice).

```rust
use kont::views::{TokenAccountView, Token2022AccountView};
use kont::primitives::{RawAccount, RawPubKey};
use kont::errors::KontError;

pub fn quick_verify_vault(
    raw_vault: &RawAccount, 
    expected_owner: &RawPubKey
) -> Result<u64, KontError> {
    // 1. Borrow the raw bytes out of the zero-dependency abstract wrapper
    let bytes = raw_vault.borrow_data()?;

    // 2. Cast the view instantly (Zero memory copy, zero heap allocations)
    // Legacy SPL Token accounts are exactly 165 bytes
    let vault_view = TokenAccountView::try_from_slice(&bytes[0..165])?;

    // 3. Offset reads executed directly via pointer jumps
    if !vault_view.owner().eq(expected_owner) {
        return Err(KontError::InvalidAccountOwner);
    }

    // 4. Return the u64 balance parsed natively from byte index [64..72]
    Ok(vault_view.amount())
}

```

### Example 2: Stack-Allocated Transfer Generation (Zero-Allocation Write)

Building an instruction template on the CPU stack, writing directly to byte coordinates, and outputting compact wire buffers.

```rust
use kont::templates::TransferTemplate;
use kont::primitives::RawPubKey;
use kont::KontInstruction;

pub fn create_wire_transfer(
    source: &RawPubKey,
    destination: &RawPubKey,
    authority: &RawPubKey,
    amount: u64,
) -> KontInstruction {
    // 1. Allocate the template directly on the stack
    let mut template = TransferTemplate::new();

    // 2. Direct memory-overwrite (blit) parameters into pre-calculated offsets
    template.set_source(source);
    template.set_destination(destination);
    template.set_authority(authority);
    template.set_amount(amount);

    // 3. Export as a unified transport-agnostic container
    // The returned KontInstruction holds the compiled 9-byte instruction payload
    template.to_kont_instruction()
}

```

### Example 3: Running Kont inside an On-Chain Program (Bridge Adapter)

If using Kont inside a standard Solana on-chain contract, write a light, local mapping translation at your entrypoint:

```rust
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use kont::primitives::{RawAccount, RawPubKey};
use kont::views::TokenAccountView;

pub fn on_chain_entrypoint_handler(vault_info: &AccountInfo) -> Result<(), ProgramError> {
    // Convert SVM native types into Kont memory interfaces at the edge
    let raw_key = RawPubKey::new(&vault_info.key.to_bytes());
    let raw_owner = RawPubKey::new(&vault_info.owner.to_bytes());
    let borrowed_data = vault_info.try_borrow_data()?;

    let raw_account = RawAccount::new(raw_key.as_bytes(), raw_owner.as_bytes(), &borrowed_data);

    // Run high-efficiency Kont operations
    let data_slice = raw_account.borrow_data().map_err(|_| ProgramError::InvalidAccountData)?;
    let view = TokenAccountView::try_from_slice(&data_slice[0..165])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    let _balance = view.amount();
    Ok(())
}

```

#### Example 4: Dynamic Token-2022 Extension Extraction (Zero-Allocation)


```rust
use kont::views::Token2022AccountView;
use kont::primitives::RawPubKey;
use kont::errors::KontError;

pub fn extract_transfer_fee_bytes<'a>(
    raw_account_data: &'a [u8]
) -> Result<Option<&'a [u8]>, KontError> {
    // 1. Wrap the entire raw buffer in the Token-2022 viewer
    let view_2022 = Token2022AccountView::try_from_slice(raw_account_data)?;

    // 2. Scan the byte array for Extension Type 3 (Transfer Fee Config)
    // Your verified TLV loop handles bounds-checking safely on the fly
    let extension_type_fee = 3;
    let fee_config_bytes = view_2022.get_extension(extension_type_fee)?;

    Ok(fee_config_bytes)
}

```

#### Example 5: High-Speed Batch Transaction Building (Multi-Mint / Burn Pipeline)

When you are arbitrage-trading or performing rapid pool balance maintenance, you often need to bundle a `MintTo` and a `Burn` instruction together. This example shows how to stack-allocate both templates and output their raw wire representations for batch dispatch.

```rust
use kont::templates::{MintToTemplate, BurnTemplate};
use kont::primitives::RawPubKey;
use kont::{KontInstruction, constants::TOKEN_PROGRAM_ID};

pub struct TokenArbitrageBundle {
    pub mint_instruction: KontInstruction,
    pub burn_instruction: KontInstruction,
}

pub fn build_arbitrage_bundle(
    mint_key: &RawPubKey,
    vault_account: &RawPubKey,
    authority: &RawPubKey,
    mint_amount: u64,
    burn_amount: u64,
) -> TokenArbitrageBundle {
    // 1. Stack-allocate the MintTo template and blit parameters
    let mut mint_stencil = MintToTemplate::new();
    mint_stencil.set_mint(mint_key);
    mint_stencil.set_destination(vault_account);
    mint_stencil.set_mint_authority(authority);
    mint_stencil.set_amount(mint_amount);

    // 2. Stack-allocate the Burn template and blit parameters
    let mut burn_stencil = BurnTemplate::new();
    burn_stencil.set_source(vault_account);
    burn_stencil.set_mint(mint_key);
    burn_stencil.set_authority(authority);
    burn_stencil.set_amount(burn_amount);

    // 3. Export both stack structures directly as raw instruction packs
    TokenArbitrageBundle {
        mint_instruction: mint_stencil.to_kont_instruction(),
        burn_instruction: burn_stencil.to_kont_instruction(),
    }
}

```

#### Example 6: High-Throughput Indexer Filter (Parsing Millions of Accounts/Sec)

If you are running an off-chain indexer parsing account states streamed from Geyser or high-volume gRPC accounts, standard deserialization is your bottleneck. Here is how to use `kont` as an ultra-fast filter layer:

```rust
use kont::views::TokenAccountView;
use kont::primitives::RawPubKey;

/// Checks if an incoming raw account state is "active" (has balance & not frozen)
/// runs at raw hardware speed with zero heap allocations.
pub fn is_active_high_value_account(
    raw_data: &[u8], 
    target_mint: &RawPubKey,
    min_balance: u64
) -> bool {
    // Perform instant boundary checks and pointer overlays
    let view = match TokenAccountView::try_from_slice(&raw_data[0..165]) {
        Ok(v) => v,
        Err(_) => return false, // Skip malformed / non-token accounts instantly
    };

    // Evaluate fields via straight memory jumps
    if !view.mint().eq(target_mint) {
        return false;
    }

    if view.is_frozen() {
        return false;
    }

    // Return true if the balance meets our searcher criteria
    view.amount() >= min_balance
}

```

---

## Implementation Reference

| Component | Target Memory Layout | CPU Mechanics |
| --- | --- | --- |
| **`RawAccount`** | Holds borrowed reference lifetimes (`'a`) | Prevents cloning raw RPC/VM arrays |
| **`TokenAccountView`** | Map overlays starting at offset `0` | Pointer jumping bounds check (`165` byte min) |
| **`Token2022AccountView`** | Map overlays with dynamic TLV scanner | Zero-allocation byte iterator over extensions |
| **`TransferTemplate`** | Optimized stack structure | Generates a 9-byte instruction payload |
| **`TransferCheckedTemplate`** | Optimized stack structure | Generates a 10-byte instruction payload |
| **`KontInstruction`** | Serialized SVM Instruction metadata | Decouples binary layout from signing client |



 
