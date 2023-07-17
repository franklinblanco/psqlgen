use std::{io::Write, fs::{File, create_dir_all}, path::PathBuf};

use crate::parser::QueryNeeds;

pub fn generate_sql_files_from_query_needs_list(query_needs_list: Vec<QueryNeeds>, output_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>>{
    
    for query_needs in query_needs_list {
        let dir_path = output_dir.join(query_needs.table_name.clone());
        create_dir_all(dir_path.clone())?;

        let insert_str = generate_insert(query_needs.clone());
        let update_str = generate_update(query_needs.clone());
        let delete_str = generate_delete(query_needs.clone());
        let select_str = generate_select(query_needs);
        
        let path_to_write = dir_path.join("insert.sql");
        if path_to_write.exists() {
            let mut file = File::create(path_to_write)?;
            file.write_all(insert_str.as_bytes())?;
        }
        

        let path_to_write = dir_path.join("update.sql");
        if path_to_write.exists() {
            let mut file = File::create(path_to_write)?;
            file.write_all(update_str.as_bytes())?;
        }

        let path_to_write = dir_path.join("delete.sql");
        if path_to_write.exists() {
            let mut file = File::create(path_to_write)?;
            file.write_all(delete_str.as_bytes())?;
        }

        let path_to_write = dir_path.join("get.sql");
        if path_to_write.exists() {
            let mut file = File::create(path_to_write)?;
            file.write_all(select_str.as_bytes())?;
        }
    }

    Ok(())
}

/// INSERT INTO 'TABLE_NAME' (...'FIELD_NAMES'...) VALUES ($1, $2, $3......) RETURNING ...'FIELD_NAMES'...;
fn generate_insert(query_needs: QueryNeeds) -> String {
    let table_name = query_needs.table_name;
    let mut joined_field_names = String::new();
    let mut parametrized_values = String::new();
    for (index, field_name) in query_needs.field_names.into_iter().enumerate() {
        joined_field_names.push_str(&format!("{field_name}, "));
        parametrized_values.push_str(&format!("${}, ", index + 1));
    }
    // This could panic if any of these two lengths = 0
    joined_field_names.remove(joined_field_names.len() - 1);
    joined_field_names.remove(joined_field_names.len() - 1);
    parametrized_values.remove(parametrized_values.len() - 1);
    parametrized_values.remove(parametrized_values.len() - 1);
    format!(
        "INSERT INTO {table_name} ({joined_field_names}) VALUES ({parametrized_values}) RETURNING {joined_field_names};"
    )
}

/// UPDATE 'TABLE_NAME' SET (...'FIELD_NAMES' = $1...) WHERE id = $X RETURNING ...'FIELD_NAMES'...;
fn generate_update(query_needs: QueryNeeds) -> String {
    let table_name = query_needs.table_name;

    let mut joined_field_names = String::new();
    for field_name in query_needs.field_names.clone() {
        joined_field_names.push_str(&format!("{field_name}, "));
    }
    // This could panic if any of these two lengths = 0
    joined_field_names.remove(joined_field_names.len() - 1);
    joined_field_names.remove(joined_field_names.len() - 1);
    let mut update_field_names_with_equals_param = String::new();
    for (index, field_name) in query_needs.field_names.into_iter().enumerate() {
        update_field_names_with_equals_param.push_str(&format!("{field_name} = ${}, ", index + 1));
    }
    // This could panic if any of these two lengths = 0
    update_field_names_with_equals_param.remove(update_field_names_with_equals_param.len() - 1);
    update_field_names_with_equals_param.remove(update_field_names_with_equals_param.len() - 1);
    format!(
        "UPDATE {table_name} SET {update_field_names_with_equals_param} WHERE id = $1 RETURNING {joined_field_names};"
    )
}

fn generate_delete(query_needs: QueryNeeds) -> String {
    let table_name = query_needs.table_name;
    format!(
        "DELETE FROM {table_name} WHERE id = $1;"
    )
}

fn generate_select(query_needs: QueryNeeds) -> String {
    let table_name = query_needs.table_name;
    let mut joined_field_names = String::new();
    let mut parametrized_values = String::new();
    for (index, field_name) in query_needs.field_names.into_iter().enumerate() {
        joined_field_names.push_str(&format!("{field_name}, "));
        parametrized_values.push_str(&format!("${}, ", index + 1));
    }
    // This could panic if any of these two lengths = 0
    joined_field_names.remove(joined_field_names.len() - 1);
    joined_field_names.remove(joined_field_names.len() - 1);
    format!(
        "SELECT {joined_field_names} FROM {table_name};"
    )
}