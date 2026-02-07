use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[wasm_bindgen]
pub fn process_compliance(
    user_access_json: &str,
    access_matrix_json: &str,
    exception_list_json: &str,
) -> Result<String, JsValue> {

    let user_access: Vec<UserAccess> =
        serde_json::from_str(user_access_json).map_err(|e| e.to_string())?;

    let access_matrix: Vec<AccessMatrix> =
        serde_json::from_str(access_matrix_json).map_err(|e| e.to_string())?;

    let exceptions: Vec<ExceptionRecord> =
        serde_json::from_str(exception_list_json).map_err(|e| e.to_string())?;

    let mut allowed = HashSet::new();
    for a in access_matrix {
        allowed.insert((a.role, a.system));
    }

    let mut exception_set = HashSet::new();
    for e in exceptions {
        exception_set.insert((e.user_id, e.system));
    }

    let mut compliant = 0;
    let mut non_compliant = 0;
    let mut violations = Vec::new();

    for ua in user_access {
        if allowed.contains(&(ua.role.clone(), ua.system.clone())) {
            compliant += 1;
        } else if exception_set.contains(&(ua.user_id.clone(), ua.system.clone())) {
            compliant += 1;
        } else {
            non_compliant += 1;
            violations.push(Violation {
                user_id: ua.user_id,
                role: ua.role,
                system: ua.system,
                status: "Non-Compliant".to_string(),
            });
        }
    }

    let output = ComplianceResult {
        violations,
        summary: vec![
            StatusCount { status: "Compliant".to_string(), count: compliant },
            StatusCount { status: "Non-Compliant".to_string(), count: non_compliant },
        ],
    };

    Ok(serde_json::to_string(&output).unwrap())
}

/* ===== DATA MODELS ===== */

#[derive(Deserialize)]
struct UserAccess {
    user_id: String,
    role: String,
    system: String,
}

#[derive(Deserialize)]
struct AccessMatrix {
    role: String,
    system: String,
}

#[derive(Deserialize)]
struct ExceptionRecord {
    user_id: String,
    system: String,
}

#[derive(Serialize)]
struct Violation {
    user_id: String,
    role: String,
    system: String,
    status: String,
}

#[derive(Serialize)]
struct StatusCount {
    status: String,
    count: u32,
}

#[derive(Serialize)]
struct ComplianceResult {
    violations: Vec<Violation>,
    summary: Vec<StatusCount>,
}
