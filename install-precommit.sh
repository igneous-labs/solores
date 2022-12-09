#!/bin/bash

wget -O pre-commit.pyz https://github.com/pre-commit/pre-commit/releases/download/v2.16.0/pre-commit-2.16.0.pyz && \
python3 pre-commit.pyz install && \
python3 pre-commit.pyz install --hook-type commit-msg && \
rm pre-commit.pyz
