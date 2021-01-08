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
ls | grep -Pv "(dist|target)" | xargs rm -rf
mv dist/* .
rm -rf dist
echo 'advent-2020.utterstep.app' > CNAME

git add . && git commit -m "[pages] update"
git push origin gh-pages --force
git checkout main
