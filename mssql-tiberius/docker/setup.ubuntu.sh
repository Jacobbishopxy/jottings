#!/usr/bin/env bash
# author:	Jacob Xie
# @date:	2023/09/15 15:36:47 Friday
# @brief:

sudo docker pull mcr.microsoft.com/mssql/server:2022-latest

sudo docker run -e "ACCEPT_EULA=Y" -e "MSSQL_SA_PASSWORD=Dev_123a" \
   --name mssql-dev -p 1433:1433 -d mcr.microsoft.com/mssql/server:2022-latest

# change password
# sudo docker exec -it mssql-dev /opt/mssql-tools/bin/sqlcmd \
#    -S localhost -U SA \
#    -P "$(read -sp "Enter current SA password: "; echo "${REPLY}")" \
#    -Q "ALTER LOGIN SA WITH PASSWORD=\"$(read -sp "Enter new SA password: "; echo "${REPLY}")\""

# ================================================================================================
# connect
# ================================================================================================

# sudo docker exec -it mssql-dev "bash"

# after enter container
# /opt/mssql-tools/bin/sqlcmd -S localhost -U SA -P "Dev_123a"
