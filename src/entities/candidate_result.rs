use diesel::QueryableByName;
use pyo3::pyclass;
use serde::Serialize;

#[pyclass]
#[derive(QueryableByName, Serialize)]
pub struct CandidateResult {
    #[pyo3(get)]
    #[sql_type = "diesel::sql_types::Integer"]
    id: i32,
    #[pyo3(get)]
    #[sql_type = "diesel::sql_types::VarChar"]
    name: String,
    #[pyo3(get)]
    #[sql_type = "diesel::sql_types::Integer"]
    applicant_id: i32,
    #[pyo3(get)]
    #[sql_type = "diesel::sql_types::Integer"]
    score: i32,
    #[pyo3(get)]
    #[sql_type = "diesel::sql_types::VarChar"]
    interview_id: String,
    #[pyo3(get)]
    #[sql_type = "diesel::sql_types::VarChar"]
    remarks: String,
    #[pyo3(get)]
    #[sql_type = "diesel::sql_types::VarChar"]
    interview_no: String,
}
