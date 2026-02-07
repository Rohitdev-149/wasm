use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[wasm_bindgen]
pub fn process_payroll(
    employee_master_json: &str,
    attendance_json: &str,
    salary_json: &str,
) -> Result<String, JsValue> {

    let employees: Vec<Employee> =
        serde_json::from_str(employee_master_json).map_err(|e| e.to_string())?;

    let attendance: Vec<Attendance> =
        serde_json::from_str(attendance_json).map_err(|e| e.to_string())?;

    let salaries: Vec<Salary> =
        serde_json::from_str(salary_json).map_err(|e| e.to_string())?;

    let mut emp_map = HashMap::new();
    for emp in employees {
        emp_map.insert(emp.employee_id.clone(), emp);
    }

    let mut sal_map = HashMap::new();
    for sal in salaries {
        sal_map.insert(sal.employee_id.clone(), sal);
    }

    let mut final_rows = Vec::new();

    for att in attendance {
        let emp = emp_map
            .get(&att.employee_id)
            .ok_or("Employee_ID missing in Employee Master")?;

        let sal = sal_map
            .get(&att.employee_id)
            .ok_or("Employee_ID missing in Salary Structure")?;

        let gross_salary = sal.basic_salary + sal.allowance;

        let final_salary = if att.days_present < 20 {
            gross_salary * 0.9
        } else {
            gross_salary
        };

        final_rows.push(FinalPayroll {
            employee_id: emp.employee_id.clone(),
            name: emp.name.clone(),
            department: emp.department.clone(),
            days_present: att.days_present,
            gross_salary,
            final_salary,
        });
    }

    Ok(serde_json::to_string(&final_rows).unwrap())
}

/* ===== DATA MODELS ===== */

#[derive(Deserialize)]
struct Employee {
    employee_id: String,
    name: String,
    department: String,
}

#[derive(Deserialize)]
struct Attendance {
    employee_id: String,
    days_present: u32,
}

#[derive(Deserialize)]
struct Salary {
    employee_id: String,
    basic_salary: f64,
    allowance: f64,
}

#[derive(Serialize)]
struct FinalPayroll {
    employee_id: String,
    name: String,
    department: String,
    days_present: u32,
    gross_salary: f64,
    final_salary: f64,
}
