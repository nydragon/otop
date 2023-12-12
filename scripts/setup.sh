#!/bin/bash

cp .hooks/* .git/hooks/;

npm install -g @commitlint/cli @commitlint/config-conventional;
