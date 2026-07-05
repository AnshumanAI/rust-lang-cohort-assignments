use std::io;

use week_04_errors_traits::*;

fn tx(txid: &str, amount_sats: u64, status: TxStatus) -> Transaction {
    Transaction {
        txid: txid.to_string(),
        amount_sats,
        status,
    }
}

fn expected_hash(material: &str) -> u64 {
    let mut hash = 0_u64;
    for byte in material.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(u64::from(byte));
    }
    hash
}

fn sample_transactions() -> Vec<Transaction> {
    vec![
        tx("tx1", 500, TxStatus::Unspent),
        tx("tx2", 200, TxStatus::Spent),
        tx("tx3", 700, TxStatus::Unspent),
    ]
}

#[test]
fn parse_status_accepts_spent() {
    assert_eq!(parse_status("spent"), Ok(TxStatus::Spent));
}

#[test]
fn parse_status_accepts_unspent() {
    assert_eq!(parse_status("unspent"), Ok(TxStatus::Unspent));
}

#[test]
fn parse_status_trims_and_ignores_case() {
    assert_eq!(parse_status(" UnSpent "), Ok(TxStatus::Unspent));
}

#[test]
fn parse_status_rejects_unknown_status() {
    assert_eq!(parse_status("pending"), Err(BtcLibError::MalformedData));
}

#[test]
fn transaction_new_copies_fields() {
    let transaction = Transaction::new("tx1", 500, TxStatus::Unspent);
    assert_eq!(transaction.txid, "tx1");
    assert_eq!(transaction.amount_sats, 500);
    assert_eq!(transaction.status, TxStatus::Unspent);
}

#[test]
fn transaction_is_unspent_matches_status() {
    assert!(tx("tx1", 1, TxStatus::Unspent).is_unspent());
    assert!(!tx("tx2", 1, TxStatus::Spent).is_unspent());
}

#[test]
fn parse_transaction_parses_valid_unspent_row() {
    assert_eq!(
        parse_transaction("tx1,500,unspent"),
        Ok(tx("tx1", 500, TxStatus::Unspent))
    );
}

#[test]
fn parse_transaction_parses_valid_spent_row() {
    assert_eq!(
        parse_transaction("tx2,200,spent"),
        Ok(tx("tx2", 200, TxStatus::Spent))
    );
}

#[test]
fn parse_transaction_trims_fields() {
    assert_eq!(
        parse_transaction(" tx1 , 500 , Unspent "),
        Ok(tx("tx1", 500, TxStatus::Unspent))
    );
}

#[test]
fn parse_transaction_rejects_missing_field() {
    assert_eq!(
        parse_transaction("tx1,500"),
        Err(BtcLibError::MalformedData)
    );
}

#[test]
fn parse_transaction_rejects_extra_field() {
    assert_eq!(
        parse_transaction("tx1,500,unspent,extra"),
        Err(BtcLibError::MalformedData)
    );
}

#[test]
fn parse_transaction_rejects_empty_txid() {
    assert_eq!(
        parse_transaction(",500,unspent"),
        Err(BtcLibError::MalformedData)
    );
}

#[test]
fn parse_transaction_rejects_empty_amount() {
    assert_eq!(
        parse_transaction("tx1,,unspent"),
        Err(BtcLibError::MalformedData)
    );
}

#[test]
fn parse_transaction_rejects_invalid_amount() {
    assert_eq!(
        parse_transaction("tx1,not-a-number,unspent"),
        Err(BtcLibError::MalformedData)
    );
}

#[test]
fn parse_transaction_rejects_zero_amount() {
    assert_eq!(
        parse_transaction("tx1,0,unspent"),
        Err(BtcLibError::MalformedData)
    );
}

#[test]
fn parse_transaction_rejects_invalid_status() {
    assert_eq!(
        parse_transaction("tx1,500,pending"),
        Err(BtcLibError::MalformedData)
    );
}

#[test]
fn parse_transaction_does_not_panic_on_bad_rows() {
    for row in [
        "",
        "tx1",
        "tx1,abc,spent",
        "tx1,1,unknown",
        "tx1,1,spent,extra",
    ] {
        let result = std::panic::catch_unwind(|| parse_transaction(row));
        assert!(result.is_ok(), "parser panicked for row: {row}");
    }
}

#[test]
fn parse_transactions_parses_many_rows() {
    assert_eq!(
        parse_transactions(&["tx1,500,unspent", "tx2,200,spent"]),
        Ok(vec![
            tx("tx1", 500, TxStatus::Unspent),
            tx("tx2", 200, TxStatus::Spent)
        ])
    );
}

#[test]
fn parse_transactions_returns_error_if_any_row_is_invalid() {
    assert_eq!(
        parse_transactions(&["tx1,500,unspent", "broken"]),
        Err(BtcLibError::MalformedData)
    );
}

#[test]
fn parse_transactions_returns_empty_vec_for_empty_input() {
    assert_eq!(parse_transactions(&[]), Ok(vec![]));
}

#[test]
fn valid_transactions_only_filters_invalid_rows() {
    assert_eq!(
        valid_transactions_only(&["tx1,500,unspent", "broken", "tx2,200,spent"]),
        vec![
            tx("tx1", 500, TxStatus::Unspent),
            tx("tx2", 200, TxStatus::Spent)
        ]
    );
}

#[test]
fn valid_transactions_only_returns_empty_vec_when_none_are_valid() {
    assert!(valid_transactions_only(&["", "broken", "tx1,0,spent"]).is_empty());
}

#[test]
fn io_errors_convert_to_btclib_error() {
    let error = io::Error::new(io::ErrorKind::Other, "disk full");
    match BtcLibError::from(error) {
        BtcLibError::Io(message) => assert!(message.contains("disk full")),
        other => panic!("expected Io error, got {other:?}"),
    }
}

#[test]
fn hash_material_uses_txid_amount_and_status_for_unspent() {
    assert_eq!(
        tx("tx1", 500, TxStatus::Unspent).hash_material(),
        "tx1:500:unspent"
    );
}

#[test]
fn hash_material_uses_txid_amount_and_status_for_spent() {
    assert_eq!(
        tx("tx2", 25, TxStatus::Spent).hash_material(),
        "tx2:25:spent"
    );
}

#[test]
fn toy_hash_hashes_hash_material() {
    let transaction = tx("tx1", 500, TxStatus::Unspent);
    assert_eq!(transaction.toy_hash(), expected_hash("tx1:500:unspent"));
}

#[test]
fn hash_all_hashes_each_item() {
    let transactions = [
        tx("tx1", 500, TxStatus::Unspent),
        tx("tx2", 25, TxStatus::Spent),
    ];
    assert_eq!(
        hash_all(&transactions),
        vec![
            expected_hash("tx1:500:unspent"),
            expected_hash("tx2:25:spent")
        ]
    );
}

#[test]
fn validate_accepts_valid_transaction() {
    assert_eq!(tx("tx1", 500, TxStatus::Unspent).validate(), Ok(()));
}

#[test]
fn validate_rejects_empty_txid() {
    assert_eq!(
        tx("", 500, TxStatus::Unspent).validate(),
        Err(BtcLibError::MalformedData)
    );
}

#[test]
fn validate_rejects_zero_amount() {
    assert_eq!(
        tx("tx1", 0, TxStatus::Unspent).validate(),
        Err(BtcLibError::MalformedData)
    );
}

#[test]
fn validate_all_accepts_all_valid_items() {
    assert_eq!(validate_all(&sample_transactions()), Ok(()));
}

#[test]
fn validate_all_rejects_first_invalid_item() {
    let transactions = [tx("tx1", 1, TxStatus::Unspent), tx("", 2, TxStatus::Spent)];
    assert_eq!(validate_all(&transactions), Err(BtcLibError::MalformedData));
}

#[test]
fn total_unspent_sums_only_unspent_transactions() {
    assert_eq!(total_unspent(&sample_transactions()), 1_200);
}

#[test]
fn find_by_txid_returns_matching_transaction() {
    assert_eq!(
        find_by_txid(&sample_transactions(), "tx2")
            .unwrap()
            .amount_sats,
        200
    );
}

#[test]
fn find_by_txid_returns_none_when_missing() {
    assert!(find_by_txid(&sample_transactions(), "missing").is_none());
}

#[test]
fn require_transaction_returns_matching_transaction() {
    assert_eq!(
        require_transaction(&sample_transactions(), "tx1")
            .unwrap()
            .status,
        TxStatus::Unspent
    );
}

#[test]
fn require_transaction_returns_error_when_missing() {
    assert_eq!(
        require_transaction(&sample_transactions(), "missing"),
        Err(BtcLibError::MissingTransaction)
    );
}

#[test]
fn summarize_amounts_counts_and_sums_by_status() {
    assert_eq!(
        summarize_amounts(&sample_transactions()),
        AmountSummary {
            count: 3,
            total_sats: 1_400,
            spent_sats: 200,
            unspent_sats: 1_200,
        }
    );
}

#[test]
fn summarize_amounts_returns_zero_summary_for_empty_input() {
    assert_eq!(
        summarize_amounts(&[]),
        AmountSummary {
            count: 0,
            total_sats: 0,
            spent_sats: 0,
            unspent_sats: 0,
        }
    );
}
