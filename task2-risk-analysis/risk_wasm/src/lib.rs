use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet};

#[wasm_bindgen]
pub fn process_risk_analysis(
    transactions_json: &str,
    master_json: &str,
) -> Result<String, JsValue> {

    let transactions: Vec<Transaction> =
        serde_json::from_str(transactions_json).map_err(|e| e.to_string())?;

    let master: Vec<MasterRecord> =
        serde_json::from_str(master_json).map_err(|e| e.to_string())?;

    let mut valid_customers = HashSet::new();
    for m in master {
        valid_customers.insert(m.customer_id);
    }

    let mut seen_tx = HashSet::new();
    let mut cleaned = Vec::new();

    let mut high_risk = 0;
    let mut medium_risk = 0;

    for tx in transactions {

        // Remove duplicates
        if !seen_tx.insert(tx.transaction_id.clone()) {
            continue;
        }

        // Remove invalid amount
        if tx.amount < 0.0 {
            continue;
        }

        let mut risk = "Low Risk";

        if tx.amount > 100000.0 {
            risk = "High Risk";
            high_risk += 1;
        } else if tx.customer_id.is_none() {
            risk = "Medium Risk";
            medium_risk += 1;
        }

        cleaned.push(CleanTransaction {
            transaction_id: tx.transaction_id,
            customer_id: tx.customer_id.unwrap_or("UNKNOWN".to_string()),
            amount: tx.amount,
            risk_level: risk.to_string(),
        });
    }

    let output = RiskOutput {
        cleaned_transactions: cleaned,
        risk_summary: vec![
            RiskCount {
                risk_type: "High Risk".to_string(),
                count: high_risk,
            },
            RiskCount {
                risk_type: "Medium Risk".to_string(),
                count: medium_risk,
            },
        ],
    };

    Ok(serde_json::to_string(&output).unwrap())
}

/* ===== MODELS ===== */

#[derive(Deserialize)]
struct Transaction {
    transaction_id: String,
    customer_id: Option<String>,
    amount: f64,
}

#[derive(Deserialize)]
struct MasterRecord {
    customer_id: String,
}

#[derive(Serialize)]
struct CleanTransaction {
    transaction_id: String,
    customer_id: String,
    amount: f64,
    risk_level: String,
}

#[derive(Serialize)]
struct RiskCount {
    risk_type: String,
    count: u32,
}

#[derive(Serialize)]
struct RiskOutput {
    cleaned_transactions: Vec<CleanTransaction>,
    risk_summary: Vec<RiskCount>,
}
