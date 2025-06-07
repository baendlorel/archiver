use owo_colors::OwoColorize;
use std::process::exit;

use crate::misc::{clap_mark, console::get_terminal_width};
use crate::traits::StripAnsi;

#[derive(Clone)]
pub struct Column {
    name: String,
    pub align: ColumnAlign,

    /// `（宽度，最大宽度）`，用来计算padding
    /// - 超过最大宽度，会缩减为省略号
    /// - 会根据rows中每一格的宽度更新
    /// - 为0的宽度不会进行padding
    width: (usize, usize),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum ColumnAlign {
    Left,
    Right,
    Center,
}

#[derive(Clone)]
pub struct TableRow {
    cells: Vec<String>,
}

pub struct Table {
    /// 表格列定义
    columns: Vec<Column>,

    /// 数据行，每一行的len必须和columns的len一致
    rows: Vec<TableRow>,

    /// 显示为表格的时候列之间的空隙
    column_space: usize,
}

impl Column {
    pub fn new(name: &str, align: ColumnAlign, width: (usize, usize)) -> Self {
        Self {
            name: name.to_string(),
            align,
            width,
        }
    }
    pub fn left_flex(name: &str) -> Self {
        Self {
            name: name.to_string(),
            align: ColumnAlign::Left,
            width: (0, 0),
        }
    }

    // 快速创建左对齐的列
    pub fn left(name: &str) -> Self {
        Self::new(name, ColumnAlign::Left, (name.len(), usize::MAX))
    }

    pub fn left_with_max(name: &str, max_width: usize) -> Self {
        Self::new(name, ColumnAlign::Left, (name.len(), max_width))
    }

    // 快速创建居中对齐的列
    pub fn center(name: &str) -> Self {
        Self::new(name, ColumnAlign::Center, (name.len(), usize::MAX))
    }
}

impl TableRow {
    pub fn new(cells: Vec<String>) -> Self {
        Self { cells }
    }
}

// lt table组件自动换行
impl Table {
    pub fn display<T>(rows: &Vec<T>)
    where
        T: TableRowify,
    {
        let table_rows = rows
            .iter()
            .map(|r| r.to_table_row())
            .collect::<Vec<TableRow>>();
        let table = Self::new(T::get_table_columns(), table_rows);
        table.display_header();
        table.display_rows();
    }

    pub fn new(mut columns: Vec<Column>, rows: Vec<TableRow>) -> Self {
        // 确保列数不为0
        if columns.is_empty() {
            panic!("{} Columns vec cannot be empty", clap_mark::fatal());
        }

        // 列空隙默认为1
        let column_space = 1;

        // 确保columns数组的width属性，只有末尾的任意个允许为0,不允许穿插0和非0
        // 因为为0的宽度不会进行padding
        let mut zero_found = false;
        for col in &columns {
            if zero_found == false && col.width.0 == 0 {
                zero_found = true;
                continue;
            }

            if zero_found && col.width.0 != 0 {
                panic!(
                    "{} Columns width cannot have non-zero after zero",
                    clap_mark::fatal()
                );
            }
        }

        // 确保每行的单元格数量与列数一致
        // 同时统一计算出列宽
        for row in &rows {
            if row.cells.len() != columns.len() {
                panic!(
                    "{} Row cell count does not match column count",
                    clap_mark::fatal()
                );
            }

            // 格式化每一列，包括预设宽度是0的最后一列
            for i in 0..columns.len() {
                columns[i].width.0 = columns[i].width.0.max(row.cells[i].true_len());

                // 最大宽度为0的是要准备flex的
                if columns[i].width.1 != 0 {
                    columns[i].width.0 = columns[i].width.0.min(columns[i].width.1);
                }
            }
        }

        // 最后一列是否自动调整宽度，触发条件为
        // - 最后一列的最大宽度为0
        // - 其他列宽度都不为0
        // 如果为true，则会根据终端宽度自动调整最后一列的宽度
        let flex_last_col = {
            let len = columns.len();
            columns[..len - 1].iter().all(|col| col.width.0 != 0) && columns[len - 1].width.1 == 0
        };

        if flex_last_col {
            let terminal_width = get_terminal_width();
            let all_other_width = columns[..columns.len() - 1]
                .iter()
                .map(|col| col.width.0)
                .sum::<usize>();
            let space = (columns.len() - 1) * column_space;

            if terminal_width <= all_other_width + space {
                println!(
                    "{} Terminal width({}) is not enough to display the data. Required: all_other_width ({}) + space ({})",
                    clap_mark::fatal(),
                    terminal_width,
                    all_other_width,
                    space
                );
                println!(
                    "{} Please stretch your terminal window and try again.",
                    clap_mark::info()
                );
                exit(1);
            }
            let last_width = terminal_width - all_other_width - space;
            let len = columns.len();
            columns[len - 1].width.0 = columns[len - 1].width.0.min(last_width);
            columns[len - 1].width.1 = columns[len - 1].width.0;
        }

        Self {
            columns,
            rows,
            column_space,
        }
    }

    pub fn format_row(&self, cells: &[String]) -> String {
        let mut formatted: Vec<String> = vec![];
        for i in 0..self.columns.len() {
            let col = &self.columns[i];
            let cell = &cells[i];
            // 宽度为0不参与padding
            if col.width.0 == 0 {
                formatted.push(cell.to_string());
                continue;
            }

            let cell_width = cell.true_len();
            // 考虑cell内容过长，省略号的情形
            let cell = if cell_width > col.width.0 {
                // 已经撑满，不需要执行下面的padding了
                formatted.push(format!(
                    "{}{}",
                    cell.omit_skip_ansi(col.width.0 - 2),
                    "..".bright_black(), // 不管怎么变化，末尾的省略号永远使用灰色
                ));
                continue;
            } else {
                cell.to_string()
            };

            let padding = if col.width.0 > cell_width {
                col.width.0 - cell_width
            } else {
                0
            };
            let s = match col.align {
                ColumnAlign::Left => format!("{}{}", cell, " ".repeat(padding)),
                ColumnAlign::Right => format!("{}{}", " ".repeat(padding), cell),
                ColumnAlign::Center => {
                    let left_pad = padding / 2;
                    let right_pad = padding - left_pad;
                    format!("{}{}{}", " ".repeat(left_pad), cell, " ".repeat(right_pad))
                }
            };
            formatted.push(s);
        }
        formatted.join(&" ".repeat(self.column_space))
    }

    pub fn display_header(&self) {
        let cells = self
            .columns
            .iter()
            .map(|col| col.name.clone())
            .collect::<Vec<String>>();
        println!("{}", &self.format_row(&cells).bold().underline());
    }

    pub fn display_rows(&self) {
        let mut rows: Vec<String> = vec![];
        for row in &self.rows {
            rows.push(self.format_row(&row.cells));
        }
        println!("{}", rows.join("\n"));
    }
}

pub trait TableRowify {
    fn to_table_row(&self) -> TableRow;

    fn get_table_columns() -> Vec<Column>;
}

#[macro_export]
macro_rules! kv_row {
    ($k:expr,$v:expr) => {
        crate::misc::console::table::TableRow::new(vec![$k.styled_field(), $v])
    };
}
