//! SQL Types.

#[derive(SqlType)]
#[diesel(postgres_type(name = "point"))]
pub struct Point;
