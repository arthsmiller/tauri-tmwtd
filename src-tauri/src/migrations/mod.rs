pub(crate) fn migrations() -> Vec<rusqlite_migration::M<'static>> {
    vec![rusqlite_migration::M::up(include_str!("./init.sql"))]
}