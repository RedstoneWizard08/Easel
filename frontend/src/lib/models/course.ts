// pub struct Model {
//     pub created_at: DateTimeWithTimeZone,
//     pub updated_at: DateTimeWithTimeZone,
//     #[sea_orm(primary_key)]
//     pub id: i32,
//     pub name: String,
//     pub code: String,
//     pub description: Option<String>,
//     pub storage_quota_bytes: Option<i64>,
//     pub owner: i32,
//     pub home_page: i32,
//     pub syllabus_page: i32,
// }

export type Course = {
    id: number;
    name: string;
    code: string;
    description?: string;
    storage_quota_bytes?: number;
    owner: number;
    home_page: number;
    syllabus_page: number;
    created_at: string;
    updated_at: string;
    banner: string; // TODO: Add this to the database for real with an actual s3 file ref
};
