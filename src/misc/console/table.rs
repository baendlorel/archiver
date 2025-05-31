use owo_colors::OwoColorize;

use crate::{misc::clap_mark, traits::StripAnsi};

// todo 编写统一的table组件
#[derive(Clone, Debug)]
pub struct Column {
    name: String,
    pub align: ColumnAlign,

    /// 列宽，用来计算padding
    /// - 会根据rows中每一格的宽度更新
    /// - 为0的宽度不会进行padding
    width: usize,
}

#[derive(Clone, Debug)]
pub enum ColumnAlign {
    Left,
    Right,
    Center,
}

pub struct TableRow {
    cells: Vec<String>,
}

pub struct Table {
    columns: Vec<Column>,
    rows: Vec<TableRow>,
}

impl Column {
    pub fn new(name: &str, align: ColumnAlign, width: usize) -> Self {
        Self {
            name: name.to_string(),
            align,
            width,
        }
    }

    pub fn with_name(name: &str) -> Self {
        Self::new(name, ColumnAlign::Left, name.len())
    }
}

impl TableRow {
    pub fn new(cells: Vec<String>) -> Self {
        Self { cells }
    }
}

impl Table {
    fn new(mut columns: Vec<Column>, rows: Vec<TableRow>) -> Self {
        // 确保列数不为0
        if columns.is_empty() {
            panic!("{} Columns vec cannot be empty", clap_mark::fatal());
        }

        // 确保columns数组的width属性，只有末尾的任意个允许为0,不允许穿插0和非0
        // 因为为0的宽度不会进行padding
        let mut zero_found = false;
        for col in &columns {
            if zero_found == false && col.width == 0 {
                zero_found = true;
                continue;
            }

            if zero_found && col.width != 0 {
                panic!(
                    "{} Columns width cannot have non-zero after zero",
                    clap_mark::fatal()
                );
            }
        }

        // 确保每行的单元格数量与列数一致
        for row in &rows {
            if row.cells.len() != columns.len() {
                panic!(
                    "{} Row cell count does not match column count",
                    clap_mark::fatal()
                );
            }

            // 宽度为0的，不格式化
            for i in 0..columns.len() {
                if columns[i].width != 0 {
                    columns[i].width = columns[i].width.max(row.cells[i].true_len());
                }
            }
        }
        Self { columns, rows }
    }

    pub fn display<T>(columns: Vec<Column>, rows: &Vec<T>)
    where
        T: TableRowify,
    {
        let table_rows = rows
            .iter()
            .map(|r| r.to_table_row())
            .collect::<Vec<TableRow>>();
        let table = Self::new(columns, table_rows);
        table.display_header();
        table.display_rows();
    }

    pub fn format_row(&self, cells: &[String]) -> String {
        let mut formatted: Vec<String> = vec![];
        for i in 0..self.columns.len() {
            let col = &self.columns[i];
            let cell = &cells[i];
            // 宽度为0不参与padding
            if col.width == 0 {
                formatted.push(cell.to_string());
                continue;
            }
            let cell_width = cell.true_len();
            let padding = if col.width > cell_width {
                col.width - cell_width
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
        formatted.join(" ")
    }

    fn display_header(&self) {
        let cells = self
            .columns
            .iter()
            .map(|col| col.name.clone())
            .collect::<Vec<String>>();
        println!("{}", &self.format_row(&cells).bold().underline());
    }

    fn display_rows(&self) {
        let mut rows: Vec<String> = vec![];
        for row in &self.rows {
            rows.push(self.format_row(&row.cells));
        }
        println!("{}", rows.join("\n"));
    }
}

pub trait TableRowify {
    fn to_table_row(&self) -> TableRow;
}
