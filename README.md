
# Kont 

**Kont** is an ultra-lean, bare-metal, `no_std` SPL Token infrastructure primitive designed for execution-critical Solana pipelines. 

By completely bypassing the standard Solana SDK (`solana-program`), dynamic heap allocations (`alloc`), and serialization frameworks (like `borsh`), **Kont** acts as a pure, header-style memory-mapping stencil. It allows you to read and write SPL Token states at raw hardware speed.

---

## Why Use Kont?

* **OS & Runtime Independence (`no_std`):** Completely decoupled from standard library file systems and networking. Run the exact same codebase on-chain, inside **AWS Nitro/Intel SGX enclaves**, or flashed directly onto low-power **embedded microcontrollers (IoT)**.
* **Minimal Compute Unit (CU) Impact:** Avoid parsing loops. Read balances, verify owners, and check mint authorities in single-digit clock cycles.
* **Stack-Allocated Blueprints (Zero Heap):** Construct, parameterize, and export binary-accurate wire instructions (`Transfer`, `Burn`, `MintTo`) using fixed stack-allocated arrays (`[u8; N]`) without triggering heap allocation.
* **Safe Pointer Casting:** Leverages safe alignment paradigms (`bytemuck`) to overlay structured byte views directly onto raw hardware addresses.

---

## Concrete Usage Examples
The following examples demonstrate how to use Kont completely independent of Solana's runtime library wrappers.
### Example 1: Fast Account Balance Inspection (Zero-Copy Read)
Overlaying the structure directly onto a raw byte buffer (e.g., from an RPC network response or on-chain memory slice).
```rust
use kont::{RawAccount, RawPubkey, TokenAccountView, KontError};

pub fn quick_verify_vault(
    raw_vault: &RawAccount, 
    expected_owner: &RawPubkey
) -> Result<u64, KontError> {
    // 1. Borrow the raw bytes out of the zero-dependency abstract wrapper
    let bytes = raw_vault.borrow_data()?;

    // 2. Cast the view instantly (Zero memory copy, zero heap allocations)
    let vault_view = TokenAccountView::try_from_slice(bytes)?;

    // 3. Offset reads executed directly via pointer jumps
    if &vault_view.owner() != expected_owner {
        return Err(KontError::InvalidAccountOwner);
    }

    // 4. Return the u64 balance parsed natively from byte index [64..72]
    Ok(vault_view.amount())
}

```
### Example 2: Stack-Allocated Transfer Generation (Zero-Allocation Write)
Building a transaction template on the CPU stack, writing directly to byte coordinates, and outputting raw instruction buffers.
```rust
use kont::templates::TransferTemplate;
use kont::{RawPubkey, KontInstruction};

pub fn create_wire_transfer(
    source: &RawPubkey,
    destination: &RawPubkey,
    authority: &RawPubkey,
    amount: u64,
) -> KontInstruction {
    // 1. Instantly allocate a fixed 114-byte array directly on the stack
    let mut template = TransferTemplate::new();

    // 2. Direct memory-overwrite (blit) parameters into pre-calculated offsets
    template.set_source(source);
    template.set_destination(destination);
    template.set_authority(authority);
    template.set_amount(amount);

    // 3. Export as a unified transport-agnostic container
    // `raw_payload` contains the completed, valid instruction bytes ready to sign & ship
    template.to_kont_instruction()
}

```
### Example 3: Running Kont inside a standard On-Chain Program (Bridge Adapter)
If using Kont inside a standard Solana on-chain contract, write a light, local mapping translation at your entrypoint:
```rust
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use kont::{RawAccount, RawPubkey, TokenAccountView};

pub fn on_chain_entrypoint_handler(vault_info: &AccountInfo) -> Result<(), ProgramError> {
    // Convert SVM native types into Kont memory interfaces at the edge
    let raw_key = RawPubkey::new(vault_info.key.to_bytes());
    let raw_owner = RawPubkey::new(vault_info.owner.to_bytes());
    let borrowed_data = vault_info.try_borrow_data()?;

    let raw_account = RawAccount::new(&raw_key, &raw_owner, &borrowed_data);

    // Run high-efficiency Kont operations
    let view = TokenAccountView::try_from_slice(raw_account.borrow_data().unwrap())
        .map_err(|_| ProgramError::InvalidAccountData)?;

    let _balance = view.amount();
    Ok(())
}

```
## Implementation Reference
The API surface area behaves like a high-precision hardware interface:
| Component | Target Memory Layout | CPU Mechanics |
|---|---|---|
| **RawAccount** | Holds borrowed reference lifetimes ('a) | Prevents cloning raw RPC/VM arrays |
| **TokenAccountView** | Map overlays starting at offset 0 | Pointer jumping bounds check (165 byte min) |
| **TransferTemplate** | Flat binary stack array ([u8; 114]) | Static const template blitting |
| **KontInstruction** | Serialized SVM Instruction wire byte array | Decouples formatting from signing/sending |
