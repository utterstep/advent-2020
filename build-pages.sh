#!/bin/bash

set -ex

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

git checkout gh-pages
git reset --hard main

cd $DIR/frontend
npm install
npm run build
cp -r dist $DIR
cd $DIR
rm -rf -- !(dist)
mv dist/* .
rm -rf dist

git add . && git commit "[pages] update"
git push origin gh-pages --force
git checkout main
