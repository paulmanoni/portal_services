use crate::entities::advert_interview::AdvertInterview;
use crate::entities::candidate_result::CandidateResult;
use crate::get_pool;
use crate::schema::advert_interviews::dsl::advert_interviews;
use crate::schema::advert_interviews::interview_id;
use diesel::{sql_query, ExpressionMethods, QueryDsl, RunQueryDsl};
use pyo3::{pyclass, pymethods, PyErr, PyResult, Python};
use pyo3::exceptions::PyValueError;

#[pyclass]
pub struct WrittenInterview;

#[pymethods]
impl WrittenInterview {
    #[new]
    fn new() -> Self {
        WrittenInterview
    }

    #[staticmethod]
    fn get_interview_info(param_id: i32) -> PyResult<AdvertInterview> {
        Python::with_gil(|_py| {
            println!("🦀 Rust Parameter: {}", param_id);
            let conn = &mut get_pool().get().unwrap();
            let result = advert_interviews
                .filter(interview_id.eq(param_id))
                .first::<AdvertInterview>(conn)
                .expect("Error loading interview");
            Ok(result)
        })
    }

    #[staticmethod]
    fn get_interview_results(param_id: i32) -> PyResult<String> {
        let conn = &mut get_pool().get().unwrap();
        println!("🦀 Rust Parameter: {}", param_id);

        let candidate_results = sql_query(
            "SELECT
                cil.id,
                CONCAT(a.first_name, ' ', a.middle_name, ' ', a.family_name) as name,
                cil.applicant_id,
                cil.score,
                cil.interview_id,
                cil.remarks,
                apl.interview_number as interview_no
            FROM candidates_interview_list cil
            JOIN applicant a ON a.applicant_id = cil.applicant_id
            JOIN applicantlist apl ON apl.applicant_list_id = cil.applicantlist_id
            WHERE cil.interview_id = ?"  // Changed ? to $1 for PostgreSQL
        )
            .bind::<diesel::sql_types::Integer, _>(param_id)
            .load::<CandidateResult>(conn)
            .map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))?;

        // Serialize to JSON
        serde_json::to_string(&candidate_results)
            .map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))
    }
}
