#!/bin/bash

# Function for displaying errors
error_exit() {
    echo -e "\n[ERROR]: $1"
    exit 1
}

# Step 1: Verify SQLite installation
echo "Checking SQLite installation..."
if ! command -v sqlite3 &> /dev/null; then
    error_exit "SQLite is not installed. Please install SQLite first."
else
    echo "SQLite found: $(sqlite3 --version)"
fi

# Step 2: Check if SQLite supports loadable extensions
echo "Checking if SQLite supports loadable extensions..."
ENABLE_LOAD_EXTENSION=$(sqlite3 ":memory:" "SELECT sqlite_compileoption_used('ENABLE_LOAD_EXTENSION');")
if [ "$ENABLE_LOAD_EXTENSION" -eq 1 ]; then
    echo "SQLite is compiled with loadable extensions support."
else
    error_exit "SQLite is not compiled with loadable extensions. Recompile SQLite with --enable-load-extension."
fi

# Step 3: Check spellfix1.so file existence and permissions
SPELLFIX_SO="./spellfix1.so"
echo "Checking spellfix1.so file existence and permissions..."
if [ ! -f "$SPELLFIX_SO" ]; then
    error_exit "The file 'spellfix1.so' does not exist in the current directory."
else
    echo "File found: $SPELLFIX_SO"
    echo "File permissions: $(ls -l $SPELLFIX_SO)"
    chmod 644 $SPELLFIX_SO || error_exit "Failed to set proper permissions on spellfix1.so"
    echo "Permissions set to 644."
fi

# Step 4: Test loading the extension manually in SQLite CLI
echo "Testing manual loading of spellfix1.so in SQLite CLI..."
sqlite3 ":memory:" ".load $SPELLFIX_SO" || error_exit "Failed to load the spellfix1.so extension manually in SQLite CLI."

echo "Manual loading of spellfix1.so successful."

# Step 5: Check loading the extension dynamically using SQL
echo "Testing dynamic loading of the spellfix1 extension..."
sqlite3 ":memory:" <<EOF || error_exit "Failed to dynamically load spellfix1 extension via SQL."
SELECT load_extension('$SPELLFIX_SO');
EOF
echo "Dynamic loading of spellfix1 successful."

# Step 6: Check extension functionality
echo "Testing the spellfix1 extension functionality..."
sqlite3 ":memory:" <<EOF || error_exit "Failed to create a virtual table using the spellfix1 extension."
SELECT load_extension('$SPELLFIX_SO');
CREATE VIRTUAL TABLE demo USING spellfix1;
INSERT INTO demo(word) VALUES('example'), ('exmple'), ('samples');
SELECT word FROM demo WHERE word MATCH 'exam';
EOF
echo "Spellfix1 extension functionality confirmed."

# Step 7: Verify linked SQLite version matches custom build
echo "Checking linked SQLite version..."
LD_LIBRARY=$(ldd $(which sqlite3) | grep "sqlite" | awk '{print $3}')
echo "SQLite library linked: $LD_LIBRARY"
if [[ "$LD_LIBRARY" != *"/custom/sqlite/path"* ]]; then
    echo "[WARNING]: SQLite may not be using the custom build with --enable-load-extension."
else
    echo "Linked library is the custom-built SQLite."
fi

echo -e "\n[INFO]: All tests completed successfully! Your SQLite setup with spellfix1 is ready to go."