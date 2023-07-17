# QueryGen
by Franklin Blanco

### *ONLY FOR POSTGRESQL At the moment*

## Goal
Feed this CLI an input of valid SQL migrations, and a directory where the output SQL Files will be generated, for [insert update delete get] SQL queries will be generated.

## How?
-Input directory
-Output directory

## Install
`cargo install psqlgen`

## Usage
Suppose we have a directory 
    migrations
that contains
        migration1.sql
        migration2.sql
And we want to generate our SQL queries in
    sql
        migration1
        migration2
We then run
`psqlgen -i migrations/ -o sql/`
Assuming valid input, This will create:
    sql
        migration1 
            insert.sql
            update.sql
            delete.sql
            get.sql
        migration2
            insert.sql
            update.sql
            delete.sql
            get.sql
