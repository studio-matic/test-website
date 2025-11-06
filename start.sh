ides start mariadb && sleep 1
mprocs "cargo run" "miniserve . --index index.html" "mariadb -S ./.ides/mariadb/run/mysqld.sock -u root"
