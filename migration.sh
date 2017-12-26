#!/bin/bash

export PATH=~/.cargo/bin:$PATH
MIGRATIONS_DIR='migrations'
PG_DATABASE_URL='postgres://ghost:ghost@localhost/ghost'
MYSQL_DATABASE_URL='mysql://ghost:ghost@localhost/ghost'
ROOT_DIR=`pwd`

################## Util ###########################
function echo_log {
    echo "`date '+%Y-%m-%d %H:%M:%S'` [$1]: $2"
}

################## Logic ###########################
function check_env {
    if [ -z `command -v diesel` ]; then
        echo_log "INFO" "not found command diesel, auto-run cargo install diesel_cli now"
        cargo install diesel_cli
    fi
}

function check_db_name() {
    case $1 in
        postgres)
            ;;
        mysql)
            ;;
        *)
          echo_log "db must belong to [postgres, mysql]"
          exit
          ;;
    esac
}

function get_db_url() {
    check_db_name $1

    if [ $1 == 'postgres' ]; then
        echo $PG_DATABASE_URL
    elif [ $1 == 'mysql' ]; then
        echo $MYSQL_DATABASE_URL
    else
        # ignore
        return
    fi
}

function get_sql() {
    sql=`cat $ROOT_DIR/sql/$1`
    echo "${sql}"
}

function has_initial() {
    if [ -d $MIGRATIONS_DIR/*"diesel_initial_setup"* ]; then
        echo 1
    else
        echo 0
    fi
}

function has_created {
    if [ -d $MIGRATIONS_DIR/*$1_create_tables* ]; then
        echo 1
    else
        echo 0
    fi
}

function create_initial_migration {
    ## pre check
    check_db_name $1 
    check_env
    if [ `has_initial` == 1 ]; then
        echo_log "INFO" "project has been initial, return"
        return
    fi
    
    db_url=`get_db_url $1`
    diesel setup --database-url $db_url
    if [ `echo $?` != 0 ]; then
        echo_log "ERROR" "diesel setup failed, maybe sql connect issue, solve it and retry again"
        exit
    fi
    if ! [ -d $MIGRATIONS_DIR ]; then
        echo_log "ERROR" "not foud folder $MIGRATIONS_DIR, maybe diesel setup failed, retry again"
        exit
    fi
}

function create_tables_migration {
    ## pre check
    check_db_name $1
    check_env
    if [ `has_created $1` == 1 ]; then
        echo_log "INFO" "$1 tables has been created, return"
        return
    fi

    ## generate migration sql
    tag=`date +%s` 
    db_name=$1
    migration_name="${db_name}_create_tables_${tag}"
    diesel migration generate $migration_name
     ## wrapper the sql into migration file
    pushd $MIGRATIONS_DIR
        for dir in *
        do
            pattern=`echo $dir | cut -d'_' -f 2-100`
            if [ $pattern == $migration_name ]; then
                pushd $dir
                    for file in *
                    do
                        if [ $file == "up.sql" ]; then 
                            get_sql "up_${db_name}.sql" >> $file
                        elif [ $file == "down.sql" ]; then
                            get_sql "down_${db_name}.sql" >> $file
                        else 
                            echo_log "ERROR" "not found up.sql or down.sql"
                        fi
                    done
                popd
            fi
        done
    popd
}

## alias `diesel migration revert`
function revert {
    check_db_name $1

    db_url=`get_db_url $1`
    diesel migration --database-url $db_url revert 
}

## alias `diesel migration run`
function run {
    check_db_name $1

    db_url=`get_db_url $1`
    diesel migration --database-url $db_url run 
}

case $1 in
    ## check enviroment
    check)
        check_env
        ;;
    ## create initial migration
    create_initial)  
        create_initial_migration $2
        ;;
    ## create tables migration
    create_tables)
        create_tables_migration $2
        ;;
    ## revert migration
    revert)
        revert $2
        ;;
    ## run migration
    run)
        run $2
        ;;
    *)
        echo "$0 [check, create_initial --db_name, create_tables --db_name, revert --db_name, run --db_name]"
        ;;
esac



