#!/bin/bash

# Load environment variables from .env file
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
else
    echo ".env file not found! Please create a .env file with a DB_FILE variable."
    exit 1
fi

# Ensure DB_FILE variable is set
if [ -z "$DATABASE_FILENAME" ]; then
    echo "DATABASE_FILENAME variable is not set in .env file!"
    exit 1
fi

# Update packages and install dependencies
sudo apt update
sudo apt install -y build-essential libsqlite3-dev tcl wget

# Download SQLite source code
wget -c https://www.sqlite.org/src/tarball/SQLite-trunk.tgz?uuid=trunk -O SQLite-trunk.tgz

# Unzip the source code
tar -xzf SQLite-trunk.tgz
cd SQLite-trunk

# Compile SQLite with FTS5 enabled
CFLAGS="-DSQLITE_ENABLE_FTS5 -DSQLITE_ENABLE_LOAD_EXTENSION" ./configure --enable-load-extension
make
sudo make install

cd ..

# Get and compile spellfix
SPELLFIX_URL="https://sqlite.org/src/raw/bcc42ef3fd29429bc01a83e751332b8d4690e65d45008449bdffe7656371487f?at=spellfix.c"
wget $SPELLFIX_URL -O spellfix1.c
gcc -shared -o spellfix1.so spellfix1.c -fPIC -I/usr/include


# Verify the installation and load the spellfix1 extension
sqlite3 --version

sqlite3 $DATABASE_FILENAME <<EOF
-- Test the spellfix1 module by creating a virtual table
CREATE VIRTUAL TABLE IF NOT EXISTS spellfix_demo USING spellfix1;
INSERT INTO spellfix_demo(word) VALUES('example'), ('exmple'), ('samples');
SELECT word FROM spellfix_demo WHERE word MATCH 'example';
EOF

# Cleanup downloaded files
rm -rf SQLite-trunk* SQLite-trunk.tgz spellfix1.c

echo "SQLite installed with FTS5 and spellfix enabled!"