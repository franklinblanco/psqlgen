#[derive(Debug)]
pub enum QueryGenError {
    InvalidPath,
    PathNotDir,
    InvalidSql,
    NotACreateTableMigration,
}
