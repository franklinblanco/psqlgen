use std::{fs, path::PathBuf};

use sqlparser::{dialect::AnsiDialect, parser::Parser};

use crate::error::QueryGenError;


#[derive(Debug, Clone)]
pub struct QueryNeeds {
    pub table_name: String,
    pub field_names: Vec<String>,
}

/// This will only check one level deep. Not recursive.
pub fn collect_sql_migrations(input_dir: PathBuf) -> Result<Vec<QueryNeeds>, QueryGenError> {
    if !input_dir.is_dir() {
        return Err(QueryGenError::PathNotDir);
    }
    let items_in_dir = match input_dir.read_dir() {
        Ok(dir) => dir,
        Err(error) => {
            println!("{error}");
            return Err(QueryGenError::InvalidPath);
        }
    };
    let mut sql_files = 0;
    let mut failed_files = 0;
    let mut query_needs_list = Vec::new();
    for item_result in items_in_dir {
        if let Ok(item) = item_result {
            if let Ok(file_type) = item.file_type() {
                if file_type.is_file() {
                    if let Ok(file_name) = item.file_name().into_string() {
                        if file_name.ends_with(".sql") {
                            sql_files += 1;
                            match fs::read_to_string(item.path()) {
                                Ok(file_contents_string) => {
                                    match collect_fields_from_migration(file_contents_string) {
                                        Ok(query_needs) => {
                                            query_needs_list.push(query_needs);
                                        },
                                        Err(error) => {
                                            failed_files += 1;
                                            println!("{:?}", error);
                                        }
                                    };
                                }
                                Err(error) => {
                                    failed_files += 1;
                                    println!("{error}");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Number of .sql Files: {sql_files}.");
    println!("Failed reading {failed_files} files.");
    Ok(query_needs_list)
}

fn collect_fields_from_migration(file_contents_string: String) -> Result<QueryNeeds, QueryGenError> {
    //let mut fields = Vec::new();

    let ast = match Parser::parse_sql(&AnsiDialect {}, &file_contents_string) {
        Ok(ast) => ast,
        Err(_) => {
            return Err(QueryGenError::InvalidSql);
        },
    };
    for stmnt in ast {
        match stmnt {
            #[allow(unused)]
            sqlparser::ast::Statement::CreateTable {
                or_replace,
                temporary,
                external,
                global,
                if_not_exists,
                transient,
                mut name,
                columns,
                constraints,
                hive_distribution,
                hive_formats,
                table_properties,
                with_options,
                file_format,
                location,
                query,
                without_rowid,
                like,
                clone,
                engine,
                default_charset,
                collation,
                on_commit,
                on_cluster,
                order_by,
                strict,
            } => {
                if name.0.len() > 0 {
                    let table_name = name.0.remove(0).value;

                    let field_names: Vec<String> = columns.into_iter().map(|col| col.name.value).collect();
                    return Ok(QueryNeeds { table_name, field_names })
                    
                } else {
                    return Err(QueryGenError::InvalidSql);
                }
                
                
                
            },
            _ => return Err(QueryGenError::NotACreateTableMigration)
            
        }
    }
    Err(QueryGenError::InvalidSql)
}

/*

[CreateTable { or_replace: false, temporary: false, external: false, global: None, if_not_exists: true, transient: false, name: ObjectName([Ident { value: "product_variant", quote_style: None }]), columns: [ColumnDef { name: Ident { value: "id", quote_style: None }, data_type: Uuid, collation: None, options: [ColumnOptionDef { name: None, option: Unique { is_primary: true } }] }, ColumnDef { name: Ident { value: "product_id", quote_style: None }, data_type: Uuid, collation: None, options: [ColumnOptionDef { name: None, option: NotNull }] }, ColumnDef { name: Ident { value: "variant_type", quote_style: None }, data_type: Varchar(None), collation: None, options: [ColumnOptionDef { name: None, option: NotNull }] }, ColumnDef { name: Ident { value: "variant_label", quote_style: None }, data_type: Varchar(None), collation: None, options: [ColumnOptionDef { name: None, option: NotNull }] }, ColumnDef { name: Ident { value: "time_created", quote_style: None }, data_type: Timestamp(None, Tz), collation: None, options: [ColumnOptionDef { name: None, option: NotNull }] }], constraints: [], hive_distribution: NONE, hive_formats: Some(HiveFormat { row_format: None, storage: None, location: None }), table_properties: [], with_options: [], file_format: None, location: None, query: None, without_rowid: false, like: None, clone: None, engine: None, default_charset: None, collation: None, on_commit: None, on_cluster: None, order_by: None, strict: false }]
*/
