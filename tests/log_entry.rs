// 为remark换行的缩进准备的常量
// 由此公式算得：字段间空格数量+状态字符数量+短横线两个
// 当前为 5+1+3
// const INVARIANT_PADDING: usize = 9;// let padding_count = time.len() + INVARIANT_PADDING + self.oper.len() + self.arg.len();
// let replacer = format!(
//     "\n{}{}{}{}{}{}",
//     "t".repeat(self.time.len()),
//     "-".repeat(5),
//     "o".repeat(self.oper.len()),
//     " ".repeat(1),
//     "a".repeat(self.arg.len()),
//     " ".repeat(3),
// );
// let replacer = format!("\n{}", " ".repeat(padding_count));
// r.replace("\n", replacer.as_str()).grey()
