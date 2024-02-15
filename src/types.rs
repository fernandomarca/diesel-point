use crate::sql_types::*;
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::prelude::*;
use std::io::Cursor;

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(sql_type = Point)]
pub struct PointXy(pub f64, pub f64);

impl PointXy {
    pub fn new(x: f64, y: f64) -> Self {
        PointXy(x, y)
    }
}

impl ToSql<Point, Pg> for PointXy {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let mut buf = Cursor::new(vec![]);
        buf.write_f64::<NetworkEndian>(self.0)?;
        buf.write_f64::<NetworkEndian>(self.1)?;
        out.write_all(&buf.into_inner())?;
        Ok(IsNull::No)
    }
}

impl ToSql<Point, Pg> for (f64, f64) {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let mut buf = Cursor::new(vec![]);
        buf.write_f64::<NetworkEndian>(self.0)?;
        buf.write_f64::<NetworkEndian>(self.1)?;
        out.write_all(&buf.into_inner())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Point, Pg> for PointXy {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let (x, y) = <(f64, f64) as FromSql<Point, Pg>>::from_sql(bytes)?;
        Ok(PointXy(x, y))
    }
}

impl FromSql<Point, Pg> for (f64, f64) {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let bytes = bytes.as_bytes();
        let mut buf = Cursor::new(bytes);
        let x = buf
            .read_f64::<NetworkEndian>()
            .map_err(|err| diesel::result::Error::DeserializationError(Box::new(err)))?;
        let y = buf
            .read_f64::<NetworkEndian>()
            .map_err(|err| diesel::result::Error::DeserializationError(Box::new(err)))?;
        Ok((x, y))
    }
}
