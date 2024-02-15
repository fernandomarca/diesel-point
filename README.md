# diesel-point &emsp; [![Latest Version]][crates.io] [![docs]][docs.rs]

[Latest Version]: https://img.shields.io/crates/v/diesel-point.svg
[crates.io]: https://crates.io/crates/diesel-point
[docs]: https://docs.rs/diesel-point/badge.svg
[docs.rs]: https://docs.rs/diesel-point

Diesel support for Point types in Postgres

### Example usage

In your sql schema, you have a column `some_point_field Point not null`.
When Diesel generates the schema (using `table! {}`) this column will look like `some_point_field -> Point`.
To ensure that the `Point` type is in scope, read [this guide](http://diesel.rs/guides/configuring-diesel-cli/) and add `use diesel_point::sql_types::*` to the `import_types` key in your `diesel.toml` file.

E.g. it will look like this:

```toml
[print_schema]
file = "src/schema.rs"

import_types = ["diesel::sql_types::*", "diesel_point::sql_types::*"]
```

In your ORM struct, write `some_point_field: PointXy`.

Now you can use this struct / table in your diesel queries.

If your table has already been created, first run diesel migration revert.
Use PointXy in the ORM struct, and then run the migration again.

Example

```rust

#[derive(Insertable, Queryable, Identifiable, Serialize, PartialEq, Debug, Clone, AsChangeset)]
#[diesel(primary_key(model_id))]
#[diesel(table_name = my_table)]
pub struct Mytable{
    pub model_id: Uuid,
    pub some_point_field: PointXy,
}

```
