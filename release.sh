#! /bin/bash
git checkout main
git merge dev
git push
git checkout release
git merge dev
git push
git checkout dev