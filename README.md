## How to test
[Install the `nix` package manager](https://nixos.org/download/) then run `nix develop . --no-pure-eval`.  
If it's your first time setting up the database, in the spawned shell run `ides start mariadb`, log into the dbms shell with `mariadb -S .ides/mariadb/run/mysqld.sock -u root` and `source ./setup.sql` and `exit` and `./start.sh offline`.  
Otherwise in the spawned shell you'll want to run just `./start.sh offline` to start the services.  

## How to deploy
[Install the `nix` package manager](https://nixos.org/download/) then run `nix develop . --no-pure-eval`.  
Run `./start.sh online` which relies on the database being set up interactively and also relies on a `DATABASE_URL=mysql://USERNAME:PASSWORD@sm-mysql-rabbit.internal/db` secret being set up.  
The front is hosted on https://studio-matic.github.io/test-website you can update it by doing `git subtree push --prefix front origin gh-pages`
