This is a local test version of which the backend is not meant to be deployed on any hosting service.

## How to run
[Install the `nix` package manager](https://nixos.org/download/) then run `nix develop . --no-pure-eval`.  
If it's your first time setting up the database, in the spawned shell run `ides start mariadb`, log into the dbms shell with `mariadb -S .ides/mariadb/run/mysqld.sock -u root` and `source ./setup.sql` and `exit` and `./start.sh`.  
Otherwise in the spawned shell you'll want to run just `./start.sh` to start the services.  
