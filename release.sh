#! /bin/bash
# if [ $? -eq 0 ]; then
#   echo "请输入合并提交的信息："
#   read msg
#   git commit -m "$msg"
# fi
# git checkout main
# git merge dev
# git push
git checkout release
git merge dev
git push
git checkout dev