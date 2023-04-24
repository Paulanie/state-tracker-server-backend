#!/bin/bash

function finish() {
        local _ret=$?
        if [[ _ret -ne 0 ]]
        then
                echo "========================================================================"
                echo "$0: ERROR: Some command exited with exit code $_ret"
                echo "========================================================================"
        fi
}
#trap finish EXIT

# This script cannot unconditionally terminate on command failure because in
# some situations command failures occurs when sqlcmd fails to log in in the
# beginning, but eventually it will succeed. Example log:
# ...
# -03-15 10:18:20.78 spid6s      Starting up database 'msdb'.
# -03-15 10:18:20.78 spid11s     Starting up database 'mssqlsystemresource'.
# -03-15 10:18:20.79 spid11s     The resource database build version is 14.00.3460. This is an informational message only. No user action is required.
# -03-15 10:18:20.84 spid11s     Starting up database 'model'.
# -03-15 10:18:21.08 spid11s     Polybase feature disabled.
# -03-15 10:18:21.08 spid11s     Clearing tempdb database.
# -03-15 10:18:21.57 spid11s     Starting up database 'tempdb'.
# -03-15 10:18:21.78 spid18s     A self-generated certificate was successfully loaded for encryption.
# -03-15 10:18:21.80 spid18s     Server is listening on [ 127.0.0.1 <ipv4> 1433].
# -03-15 10:18:21.80 spid18s     Dedicated administrator connection support was not started because it is disabled on this edition of SQL Server. If you want to use a dedicated administrator connection, re
# start SQL Server using the trace flag 7806. This is an informational message only. No user action is required.
# -03-15 10:18:21.87 Logon       Error: 18401, Severity: 14, State: 1.
# -03-15 10:18:21.87 Logon       Login failed for user 'sa'. Reason: Server is in script upgrade mode. Only administrator can connect at this time. [CLIENT: 127.0.0.1]
# Waiting for sys.databases query to return 4 or more rows
# -03-15 10:18:21.94 spid6s      Synchronize Database 'msdb' (4) with Resource Database.
# -03-15 10:18:21.94 spid21s     The Service Broker endpoint is in disabled or stopped state.
# -03-15 10:18:21.94 spid21s     The Database Mirroring endpoint is in disabled or stopped state.
# -03-15 10:18:22.04 spid21s     Service Broker manager has started.
# -03-15 10:18:22.24 spid6s      Database 'master' is upgrading script 'u_tables.sql' from level 234884480 to level 234884484.
# -03-15 10:18:22.25 spid6s      Starting u_Tables.SQL at  15 Mar 2023 10:18:22:253
# -03-15 10:18:22.25 spid6s      This file creates all the system tables in master.
# -03-15 10:18:22.28 spid6s      drop view spt_values ....
# -03-15 10:18:22.34 spid6s      Creating view 'spt_values'.
# -03-15 10:18:22.46 spid6s      drop table spt_monitor ....
# -03-15 10:18:22.50 spid6s      Creating 'spt_monitor'.
# -03-15 10:18:22.52 spid6s      Grant Select on spt_monitor
# -03-15 10:18:22.53 spid6s      Insert into spt_monitor ....
# -03-15 10:18:22.89 spid6s      Finishing at  15 Mar 2023 10:18:22:893
# -03-15 10:18:22.91 Logon       Error: 18401, Severity: 14, State: 1.
# -03-15 10:18:22.91 Logon       Login failed for user 'sa'. Reason: Server is in script upgrade mode. Only administrator can connect at this time. [CLIENT: 127.0.0.1]
# Waiting for sys.databases query to return 4 or more rows
# -03-15 10:18:23.08 spid6s      Database 'master' is upgrading script 'ProvisionAgentIdentity.sql' from level 234884480 to level 234884484.
# -03-15 10:18:23.08 spid6s      Database 'master' is upgrading script 'no_op.sql' from level 234884480 to level 234884484.
# -03-15 10:18:23.33 spid6s      Database 'master' is upgrading script 'no_op.sql' from level 234884480 to level 234884484.
# -03-15 10:18:23.33 spid6s      -----------------------------------------
# -03-15 10:18:23.33 spid6s      Starting execution of dummy.sql
# -03-15 10:18:23.33 spid6s      -----------------------------------------
# -03-15 10:18:23.34 spid6s      Database 'master' is upgrading script 'repl_upgrade.sql' from level 234884480 to level 234884484.
# -03-15 10:18:23.34 spid6s      Executing replication upgrade scripts.
# -03-15 10:18:23.49 spid6s      Attempting to load library 'xpstar.dll' into memory. This is an informational message only. No user action is required.
# -03-15 10:18:23.53 spid6s      Using 'xpstar.dll' version '2017.140.3460' to execute extended stored procedure 'xp_instance_regread'. This is an informational message only; no user action is required.
# -03-15 10:18:23.53 spid6s      Executing sp_vupgrade_replication.
# -03-15 10:18:23.57 spid6s      DBCC execution completed. If DBCC printed error messages, contact your system administrator.
# -03-15 10:18:23.93 spid6s      Starting up database 'SomeDatabase1'.
# -03-15 10:18:23.96 Logon       Error: 18401, Severity: 14, State: 1.
# -03-15 10:18:23.96 Logon       Login failed for user 'sa'. Reason: Server is in script upgrade mode. Only administrator can connect at this time. [CLIENT: 127.0.0.1]
# Waiting for sys.databases query to return 4 or more rows
# -03-15 10:18:24.04 spid6s      Parallel redo is started for database 'SomeDatabase1' with worker pool size [1].
# -03-15 10:18:24.07 spid6s      Parallel redo is shutdown for database 'SomeDatabase1' with worker pool size [1].
# -03-15 10:18:24.15 spid6s      Synchronize Database 'SomeDatabase1' (6) with Resource Database.
# -03-15 10:18:24.19 spid6s      Starting up database 'SomeDatabase2'.
# -03-15 10:18:24.33 spid6s      Parallel redo is started for database 'SomeDatabase2' with worker pool size [1].
# -03-15 10:18:24.75 spid6s      Parallel redo is shutdown for database 'SomeDatabase2' with worker pool size [1].
# -03-15 10:18:24.87 spid6s      Synchronize Database 'SomeDatabase2' (7) with Resource Database.
# ...
# -03-15 10:19:25.75 spid6s      The Utility MDW does not exist on this instance.
# -03-15 10:19:25.75 spid6s      User 'sa' is changing database script level entry 15 to a value of 500.
# -03-15 10:19:25.76 spid6s      Skipping the execution of instmdw.sql.
# -03-15 10:19:25.76 spid6s      ------------------------------------------------------
# -03-15 10:19:25.76 spid6s      execution of UPGRADE_UCP_CMDW_DISCOVERY.SQL completed
# -03-15 10:19:25.76 spid6s      ------------------------------------------------------
# -03-15 10:19:25.84 spid6s      Database 'master' is upgrading script 'ssis_discovery' from level 234884480 to level 234884484.
# -03-15 10:19:25.84 spid6s      ------------------------------------------------------
# -03-15 10:19:25.85 spid6s      Starting execution of SSIS_DISCOVERY.SQL
# -03-15 10:19:25.85 spid6s      ------------------------------------------------------
# -03-15 10:19:25.88 spid6s      Database SSISDB does not exist in current SQL Server instance
# -03-15 10:19:25.88 spid6s      User 'sa' is changing database script level entry 17 to a value of 500.
# -03-15 10:19:25.88 spid6s      Database SSISDB could not be upgraded successfully.
# -03-15 10:19:25.88 spid6s      ------------------------------------------------------
# -03-15 10:19:25.89 spid6s      Execution of SSIS_DISCOVERY.SQL completed
# -03-15 10:19:25.89 spid6s      ------------------------------------------------------
# -03-15 10:19:25.97 spid6s      Database 'master' is upgrading script 'SSIS_hotfix_install.sql' from level 234884480 to level 234884484.
# -03-15 10:19:26.00 spid6s      ------------------------------------------------------
# -03-15 10:19:26.00 spid6s      Starting execution of SSIS_HOTFIX_INSTALL.SQL
# -03-15 10:19:26.01 spid6s      ------------------------------------------------------
# -03-15 10:19:26.01 spid6s      Database SSISDB does not exist in current SQL Server instance
# -03-15 10:19:26.01 spid6s      ------------------------------------------------------
# -03-15 10:19:26.01 spid6s      Execution of SSIS_HOTFIX_INSTALL.SQL completed
# -03-15 10:19:26.01 spid6s      ------------------------------------------------------
# -03-15 10:19:26.01 spid6s      Database 'master' is upgrading script 'provision_ceipsvc_account.sql' from level 234884480 to level 234884484.
# -03-15 10:19:26.02 spid6s      ------------------------------------------------------
# -03-15 10:19:26.02 spid6s      Start provisioning of CEIPService Login
# -03-15 10:19:26.02 spid6s      ------------------------------------------------------
# -03-15 10:19:26.03 spid6s      Configuration option 'show advanced options' changed from 0 to 1. Run the RECONFIGURE statement to install.
# -03-15 10:19:26.03 spid6s      Configuration option 'show advanced options' changed from 0 to 1. Run the RECONFIGURE statement to install.
# -03-15 10:19:26.05 spid6s      Configuration option 'Agent XPs' changed from 0 to 1. Run the RECONFIGURE statement to install.
# -03-15 10:19:26.05 spid6s      Configuration option 'Agent XPs' changed from 0 to 1. Run the RECONFIGURE statement to install.
# -03-15 10:19:26.06 spid6s      Configuration option 'Agent XPs' changed from 1 to 0. Run the RECONFIGURE statement to install.
# -03-15 10:19:26.06 spid6s      Configuration option 'Agent XPs' changed from 1 to 0. Run the RECONFIGURE statement to install.
# -03-15 10:19:26.06 spid6s      Configuration option 'show advanced options' changed from 1 to 0. Run the RECONFIGURE statement to install.
# -03-15 10:19:26.07 spid6s      Configuration option 'show advanced options' changed from 1 to 0. Run the RECONFIGURE statement to install.
# -03-15 10:19:26.07 spid6s      ------------------------------------------------------
# -03-15 10:19:26.07 spid6s      Ending provisioning of CEIPLoginName.
# -03-15 10:19:26.08 spid6s      ------------------------------------------------------
# -03-15 10:19:26.08 spid6s      SQL Server is now ready for client connections. This is an informational message; no user action is required.
# -03-15 10:19:26.08 spid6s      Recovery is complete. This is an informational message only. No user action is required.
# ========================================================================
# Database ready for initialization
# ========================================================================
# Processing create_login_and_user.sql
# ...


# So starting with termination enabled but, selectively turning off later on.
set -euo pipefail

# /opt/mssql/bin/sqlservr ought to match the CMD from the upstream docker image, e.g.
#
# $ docker inspect mcr.microsoft.com/mssql/server:2017-latest | jq '.[0].Config.Cmd'
# [
#   "/opt/mssql/bin/sqlservr"
# ]
# $

function wait_for_server_ready() {
        local rows_affected=0

        # Sql server has 4 system database, so when they are connected and
        # created, SQL server can be assumed to be running and working as
        # expected. Example output from sqlcmd:

        #name
        #--------------------------------------------------------------------------------------------------------------------------------
        #master
        #tempdb
        #model
        #msdb
        #some_other_database_001
        #some_other_database_002
        #some_other_database_003
        #some_other_database_004
        #some_other_database_005
        #
        #(9 rows affected)
        while [[ rows_affected -lt 4 ]]
        do
                echo "Waiting for sys.databases query to return 4 or more rows"
                sleep 1
                set +o pipefail # Turn off termination for sqlcmd failures, see earlier comments.
                rows_affected=$(/opt/mssql-tools/bin/sqlcmd -S 127.0.0.1 -U sa -P "${MSSQL_SA_PASSWORD}" -Q 'SELECT name FROM sys.databases' 2>/dev/null | sed -n 's/(//; s/ rows affected.*//p;')
                set -o pipefail
        done
}

function process_init_scripts() {
        local old_pwd=$PWD
        for dir in "$@"
        do
                cd "$dir" || { echo "$0; Error: unable to cd to $dir" 1>&2; exit 1;}
                for file in *.sql
                do
                        case "$file" in
                                "*.sql")
                                        echo "No *.sql files in $dir."
                                        ;;
                                *)
                                        echo "Processing $file"
                                        /opt/mssql-tools/bin/sqlcmd -S 127.0.0.1 -U sa -P "${MSSQL_SA_PASSWORD}" -d master -i "$file"
                                        sync
                                        ;;
                        esac
                done
        done
        cd "$old_pwd"
}

echo "Starting SQL-Server on 127.0.0.1"
# Ignoring output from mssql-conf since it will complain
#   SQL Server needs to be restarted in order to apply this setting. Please run
#   'systemctl restart mssql-server.service'.
# which is not an issue here in this script.
/opt/mssql/bin/mssql-conf set network.ipaddress 127.0.0.1 > /dev/null
/opt/mssql/bin/sqlservr &
pid=$!

wait_for_server_ready
echo "========================================================================"
echo "Database ready for initialization"
echo "========================================================================"
process_init_scripts /docker-entrypoint-initdb.d

sync

echo "Setup finished. Stopping SQL-Server..."
kill "$pid"
wait "$pid"

sync
sleep 1

echo "Starting SQL-Server on 0.0.0.0"
/opt/mssql/bin/mssql-conf set network.ipaddress 0.0.0.0 > /dev/null

# NB NB The last (long running) command should always be started through exec, see
# https://stackoverflow.com/questions/39082768/what-does-set-e-and-exec-do-for-docker-entrypoint-scripts and
# https://stackoverflow.com/questions/32255814/what-purpose-does-using-exec-in-docker-entrypoint-scripts-serve
# for details.
exec /opt/mssql/bin/sqlservr