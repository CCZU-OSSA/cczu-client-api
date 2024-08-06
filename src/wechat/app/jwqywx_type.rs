use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Message<T> {
    pub status: i32,
    pub message: Vec<T>,
    pub token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LoginUserData {
    #[serde(rename = "yhdm")]
    pub userid: String,
    #[serde(rename = "yhmc")]
    pub username: String,
    #[serde(rename = "yhsf")]
    pub userident: String,
    #[serde(rename = "xq")]
    pub term: String,
    #[serde(rename = "dqz")]
    pub current_value: i32,
    #[serde(rename = "zc")]
    pub position: i32,
    #[serde(rename = "gh")]
    pub employee_number: String,
    pub smscode: String,
    #[serde(rename = "xb")]
    pub gender: String,
    #[serde(rename = "yhqx")]
    pub permission: String,
}

#[derive(Deserialize, Debug)]
pub struct CourseGrade {
    #[serde(rename = "bh")]
    pub class_id: String,
    #[serde(rename = "bj")]
    pub class_name: String,
    #[serde(rename = "xh")]
    pub student_id: String,
    #[serde(rename = "xm")]
    pub student_name: String,
    #[serde(rename = "kcdm")]
    pub course_id: String,
    #[serde(rename = "kcmc")]
    pub course_name: String,
    #[serde(rename = "xq")]
    pub term: i32,
    #[serde(rename = "kclb")]
    pub course_type: String,
    #[serde(rename = "lbmc")]
    pub course_type_name: String,
    #[serde(rename = "xs")]
    pub course_hours: i32,
    #[serde(rename = "xf")]
    pub course_credits: f32,
    #[serde(rename = "jsmc")]
    pub teacher_name: String,
    #[serde(rename = "ksxzm")]
    pub is_exam_type: i32,
    #[serde(rename = "ksxz")]
    pub exam_type: String,
    #[serde(rename = "pscj")]
    pub usual_grade: f32,
    #[serde(rename = "qzcj")]
    pub mid_exam_grade: f32,
    #[serde(rename = "qmcj")]
    pub end_exam_grade: f32,
    #[serde(rename = "kscj")]
    pub exam_grade: String,
    #[serde(rename = "idn")]
    pub ident: i32,
    #[serde(rename = "cj")]
    pub grade: f32,
    #[serde(rename = "xfjd")]
    pub grade_points: f32,
}

#[derive(Deserialize, Debug)]
pub struct StudentPoint {
    #[serde(rename = "nj")]
    pub grade: String,
    #[serde(rename = "bh")]
    pub class_id: String,
    #[serde(rename = "bj")]
    pub class_name: String,
    #[serde(rename = "xh")]
    pub student_id: String,
    #[serde(rename = "xm")]
    pub student_name: String,
    #[serde(rename = "xfjd")]
    pub grade_points: String,
    #[serde(rename = "pm")]
    pub rank: String,
    #[serde(rename = "zypm")]
    pub major_rank: String,
    #[serde(rename = "zxfjd")]
    pub total_grade_points: String,
    #[serde(rename = "zxf")]
    pub total_credits: String,
    #[serde(rename = "pjcjfx")]
    pub average_credits: String,
    #[serde(rename = "pjxfjd")]
    pub average_grade_points: String,
    #[serde(rename = "pjcj")]
    pub average_grade: String,
}
