#!/usr/bin/env bash
# author:	Jacob Xie
# @date:	2023/09/16 16:01:21 Saturday
# @brief:

sudo docker pull mcr.microsoft.com/azure-sql-edge:latest

sudo docker run -e "ACCEPT_EULA=Y" -e "MSSQL_SA_PASSWORD=Dev_123a" \
   --name mssql-dev -p 1433:1433 -d mcr.microsoft.com/azure-sql-edge:latest

