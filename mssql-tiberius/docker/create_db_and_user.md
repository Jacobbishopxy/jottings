# Create DB and User

```sh
sudo docker exec -it mssql-dev "bash"
```

after enter container:

```sh
/opt/mssql-tools/bin/sqlcmd -S localhost -U SA -P "Dev_123a"
```

```sql
-- create database and new login
CREATE DATABASE devdb;
CREATE LOGIN dev WITH PASSWORD = 'StrongPassword123';
GO

-- switch to database and create new user with login
USE devdb
CREATE USER dev FOR LOGIN dev;
GO

-- grant owner to user
EXEC sp_addrolemember 'db_owner', 'dev';
GO
```
