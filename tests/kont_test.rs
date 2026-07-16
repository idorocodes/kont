use kont::{ 
    constant::TOKEN_PROGRAM_ID, errors::KontError, primitives::{RawAccount, RawPubKey}, templates::{mint::{BurnTemplate, MintToTemplate}, transfer::{TransferCheckedTemplate, TransferTemplate}}, views::{token_2022::Token2022AccountView, token_v1::TokenAccountView},
};



const fn mock_pubkey(marker: u8) -> RawPubKey {
    let mut bytes = [0u8; 32];
    bytes[0] = marker;
    RawPubKey::new(&bytes)
}

// =========================================================================
// 1. Primitive Tests (RawPubKey & RawAccount)
// =========================================================================

#[test]
fn test_raw_pubkey_equality_and_bytes() {
    let key_a1 = mock_pubkey(0xAA);
    let key_a2 = mock_pubkey(0xAA);
    let key_b = mock_pubkey(0xBB);

    assert!(key_a1.eq(&key_a2));
    assert!(!key_a1.eq(&key_b));
    assert_eq!(key_a1.as_bytes()[0], 0xAA);
}

#[test]
fn test_raw_account_memory_binding() {
    let key = mock_pubkey(0x11);
    let owner = mock_pubkey(0x22);
    let raw_data = [0xDE, 0xAD, 0xBE, 0xEF];

    let key = RawPubKey::new(key.as_bytes());
    let binding = RawPubKey::new(owner.as_bytes());
    let account = RawAccount::new(&key, &binding, &raw_data);

    assert_eq!(account.key().as_bytes(), key.as_bytes());
    assert_eq!(account.owner().as_bytes(), owner.as_bytes());
    assert_eq!(account.borrow_data().unwrap(), &raw_data);
}

// =========================================================================
// 2. Token V1 Zero-Copy Parsing Tests
// =========================================================================

#[test]
fn test_token_v1_account_view_bounds_and_fields() {
    let mut mock_data = [0u8; 165];

    let mint = mock_pubkey(0x01);
    let owner = mock_pubkey(0x02);
    let amount: u64 = 42_000_000;
    let delegate = mock_pubkey(0x03);

    mock_data[0..32].copy_from_slice(mint.as_bytes());
    mock_data[32..64].copy_from_slice(owner.as_bytes());
    mock_data[64..72].copy_from_slice(&amount.to_le_bytes());
    
    mock_data[72..76].copy_from_slice(&1u32.to_le_bytes());
    mock_data[76..108].copy_from_slice(delegate.as_bytes());
    mock_data[108] = 2; // Frozen

    let view = TokenAccountView::try_from_slice(&mock_data).expect("Parsing must succeed");

    assert_eq!(view.mint().as_bytes(), mint.as_bytes());
    assert_eq!(view.owner().as_bytes(), owner.as_bytes());
    assert_eq!(view.amount(), 42_000_000);
    assert_eq!(view.delegate().unwrap().as_bytes(), delegate.as_bytes());
    assert!(view.is_frozen());
}

#[test]
fn test_token_v1_optional_delegate_none() {
    let mut mock_data = [0u8; 165];
    mock_data[72..76].copy_from_slice(&0u32.to_le_bytes());

    let view = TokenAccountView::try_from_slice(&mock_data).unwrap();
    assert!(view.delegate().is_none());
}

// =========================================================================
// 3. Token-2022 Dynamic TLV Extensions parsing Tests
// =========================================================================

#[test]
fn test_token_2022_dynamic_tlv_scans() {
    // Generate buffer exceeding base size
    let mut mock_data = [0u8; 256];
    
    // Initialize base legacy parameters (first 165 bytes)
    let mint = mock_pubkey(0x10);
    mock_data[0..32].copy_from_slice(mint.as_bytes());
    mock_data[72..76].copy_from_slice(&0u32.to_le_bytes()); // Delegate = None (0)

    // Set up TLV segments starting at index 166 (matching your scanner's loop initial offset)
    let ext_type_a: u16 = 3;  
    let ext_len_a: u16 = 8;
    let ext_val_a: [u8; 8] = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x11, 0x22];

    let ext_type_b: u16 = 7;  
    let ext_len_b: u16 = 4;
    let ext_val_b: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];

    // Align start offset to match your physical Token2022AccountView setup (166)
    let start_offset = 166; 
    
    // Pack extension A
    let mut curr = start_offset;
    mock_data[curr..curr+2].copy_from_slice(&ext_type_a.to_le_bytes());
    mock_data[curr+2..curr+4].copy_from_slice(&ext_len_a.to_le_bytes());
    mock_data[curr+4..curr+12].copy_from_slice(&ext_val_a);

    // Pack extension B
    curr += 12; // 2 (type) + 2 (len) + 8 (val)
    mock_data[curr..curr+2].copy_from_slice(&ext_type_b.to_le_bytes());
    mock_data[curr+2..curr+4].copy_from_slice(&ext_len_b.to_le_bytes());
    mock_data[curr+4..curr+8].copy_from_slice(&ext_val_b);

    let view2022 = Token2022AccountView::try_from_slice(&mock_data[0..256]).expect("Should wrap safely");

    // Let's resolve the base() extraction safely. 
    let base_view = view2022.base().expect("Base view parsing failed - check your layout validation rules");
    assert_eq!(base_view.mint().as_bytes(), mint.as_bytes());

    let fetched_b = view2022.get_extension(ext_type_b).unwrap().expect("Should locate ext B");
    assert_eq!(fetched_b, &ext_val_b);

    let fetched_a = view2022.get_extension(ext_type_a).unwrap().expect("Should locate ext A");
    assert_eq!(fetched_a, &ext_val_a);

    let missing_ext = view2022.get_extension(99).unwrap();
    assert!(missing_ext.is_none());
}

#[test]
fn test_token_2022_corrupted_tlv_length_returns_overflow_error() {
    // A malicious or corrupted account could declare an extension length
    // that reads past the end of the actual buffer. The scanner must catch
    // this and return TlvParsingOverflow instead of panicking or reading OOB.
    let mut mock_data = [0u8; 200];

    let start_offset = 166;
    let ext_type: u16 = 3;
    // Declare a length far larger than the remaining buffer actually has.
    let bogus_len: u16 = 255;

    mock_data[start_offset..start_offset + 2].copy_from_slice(&ext_type.to_le_bytes());
    mock_data[start_offset + 2..start_offset + 4].copy_from_slice(&bogus_len.to_le_bytes());
    // Deliberately no value payload written - buffer is too short for `bogus_len`.

    let view2022 = Token2022AccountView::try_from_slice(&mock_data).expect("Should wrap safely");

    let result = view2022.get_extension(ext_type);
    assert_eq!(result, Err(KontError::TlvParsingOverflow));
}
// =========================================================================
// 4. Stencil-Based Stack Template (Write) Tests
// =========================================================================

#[test]
fn test_transfer_instruction_stencil_blitting() {
    let source = mock_pubkey(0x11);
    let destination = mock_pubkey(0x22);
    let authority = mock_pubkey(0x33);
    let amount: u64 = 50_000;

    let mut template = TransferTemplate::new();
    template.set_source(&source);
    template.set_destination(&destination);
    template.set_authority(&authority);
    template.set_amount(amount);

    let ix = template.to_kont_instruction();

    assert_eq!(ix.program_id().as_bytes(), TOKEN_PROGRAM_ID.as_bytes());
    assert_eq!(ix.account_count(), 3);
    
    assert_eq!(ix.accounts[0].pubkey.as_bytes(), source.as_bytes());
    assert_eq!(ix.accounts[1].pubkey.as_bytes(), destination.as_bytes());
    assert_eq!(ix.accounts[2].pubkey.as_bytes(), authority.as_bytes());

    let bytes = ix.as_bytes();
    assert_eq!(bytes.len(), 9); // Standard Transfer template structure layout length
    assert_eq!(bytes[0], 3); 
}

#[test]
fn test_transfer_checked_stencil_blitting() {
    let source = mock_pubkey(0xAA);
    let mint = mock_pubkey(0xBB);
    let destination = mock_pubkey(0xCC);
    let authority = mock_pubkey(0xDD);
    let amount: u64 = 750;
    let decimals: u8 = 9;

    let mut template = TransferCheckedTemplate::new();
    template.set_source(&source);
    template.set_mint(&mint);
    template.set_destination(&destination);
    template.set_authority(&authority);
    template.set_amount(amount);
    template.set_decimals(decimals);

    let ix = template.to_kont_instruction();
    
    assert_eq!(ix.account_count(), 4);
    assert_eq!(ix.accounts[0].pubkey.as_bytes(), source.as_bytes());
    assert_eq!(ix.accounts[1].pubkey.as_bytes(), mint.as_bytes());
    assert_eq!(ix.accounts[2].pubkey.as_bytes(), destination.as_bytes());
    assert_eq!(ix.accounts[3].pubkey.as_bytes(), authority.as_bytes());

    let bytes = ix.as_bytes();
    assert_eq!(bytes.len(), 10); // Standard TransferChecked template layout length
    assert_eq!(bytes[0], 12); 
}

#[test]
fn test_mint_to_template_assembly() {
    let mint = mock_pubkey(0x55);
    let destination = mock_pubkey(0x66);
    let authority = mock_pubkey(0x77);
    let amount: u64 = 1_234_567_890;

    let mut stencil = MintToTemplate::new();
    stencil.set_mint(&mint);
    stencil.set_destination(&destination);
    stencil.set_mint_authority(&authority);
    stencil.set_amount(amount);

    let ix = stencil.to_kont_instruction();

    assert_eq!(ix.account_count(), 3);
    assert_eq!(ix.accounts[0].pubkey.as_bytes(), mint.as_bytes());
    assert_eq!(ix.accounts[1].pubkey.as_bytes(), destination.as_bytes());
    assert_eq!(ix.accounts[2].pubkey.as_bytes(), authority.as_bytes());

    // Verified: Expecting the 105-byte physical buffer size defined in src/templates/mint.rs
    let bytes = ix.as_bytes();
    assert_eq!(bytes.len(), 105); 
    assert_eq!(bytes[0], 7); // MintTo discriminant

    // Amount is mapped at index 97..105 in your stencil implementation
    let parsed_amount = u64::from_le_bytes(bytes[97..105].try_into().unwrap());
    assert_eq!(parsed_amount, amount);
}

#[test]
fn test_burn_template_assembly() {
    let source = mock_pubkey(0x88);
    let mint = mock_pubkey(0x99);
    let authority = mock_pubkey(0xAA);
    let amount: u64 = 987_654_321;

    let mut stencil = BurnTemplate::new();
    stencil.set_source(&source);
    stencil.set_mint(&mint);
    stencil.set_authority(&authority);
    stencil.set_amount(amount);

    let ix = stencil.to_kont_instruction();

    assert_eq!(ix.account_count(), 3);
    assert_eq!(ix.accounts[0].pubkey.as_bytes(), source.as_bytes());
    assert_eq!(ix.accounts[1].pubkey.as_bytes(), mint.as_bytes());
    assert_eq!(ix.accounts[2].pubkey.as_bytes(), authority.as_bytes());

    // Verified: Expecting the 105-byte physical buffer size defined in src/templates/mint.rs
    let bytes = ix.as_bytes();
    assert_eq!(bytes.len(), 105); 
    assert_eq!(bytes[0], 8); // Burn discriminant

    // Amount is mapped at index 97..105 in your stencil implementation
    let parsed_amount = u64::from_le_bytes(bytes[97..105].try_into().unwrap());
    assert_eq!(parsed_amount, amount);
}