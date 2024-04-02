use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct GradeData {
    pub name: String,
    pub point: String,
    pub grade: String,
}
