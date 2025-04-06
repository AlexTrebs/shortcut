#!/bin/bash

cargo install sqlx-cli
sqlx db create
sqlx migrate run