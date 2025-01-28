diesel::table! {
    advert_interviews (interview_id){
        interview_id -> Integer,
        advert_id -> Integer,
        interview_date -> Datetime,
        venue -> Nullable<VarChar>,
        interview_category -> Nullable<VarChar>,
        cutoff_marks -> Nullable<Integer>,
        status -> Nullable<Integer>,
        is_final -> Nullable<Integer>,
    }
}