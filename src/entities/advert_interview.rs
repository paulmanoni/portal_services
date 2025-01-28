use diesel::{Queryable, Selectable};
use pyo3::{pyclass};

#[pyclass]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::advert_interviews)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct AdvertInterview {
    #[pyo3(get)]
    #[diesel(column_name = interview_id)]
    pub(crate) id: i32,
    #[pyo3(get)]
    pub(crate) advert_id: i32,
    pub(crate) interview_date: chrono::NaiveDateTime,
    #[pyo3(get)]
    pub(crate) venue: Option<String>,
    #[pyo3(get)]
    pub(crate) interview_category: Option<String>,
    #[pyo3(get)]
    pub(crate) cutoff_marks: Option<i32>,
    #[pyo3(get)]
    pub(crate) status: Option<i32>,
    #[pyo3(get)]
    pub(crate) is_final: Option<i32>
}

