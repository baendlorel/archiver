#!/bin/bash

# 生成临时文件
tmpfile=$(mktemp)

# 定义所有标签（正则）
pattern='(//|#)[[:space:]]*(TODO|FIXME|HACK|LT)'

# 提取所有标签内容并去重
git log --pretty=format:'%ad %H' --date=short | while read date hash; do
    git grep -i -h -E "$pattern" $hash | sed 's/^[[:space:]]*//;s/[[:space:]]*$//' | awk -v d="$date" '{print d "|" $0}'
done | sort -t'|' -k2,2 -k1,1 | awk -F'|' '!seen[$2]++{print $1, $2}' | sort > "$tmpfile"

# ...existing code...

echo "所有历史标签内容已写入: $tmpfile"

# 分类统计
echo -e "\n分类统计："
awk -F'|' '{
    content = substr($0, index($0, $2))
    lc = tolower(content)
    if (lc ~ /\/ todo/)   {todo++}
    if (lc ~ /\/ fixme/)  {fixme++}
    if (lc ~ /\/ hack/)   {hack++}
    if (lc ~ /\/ lt/)     {lt++}
}
END {
    printf "TODO:   %d\nFIXME:  %d\nHACK:   %d\nLT:     %d\n", todo, fixme, hack, lt
}' "$tmpfile"

