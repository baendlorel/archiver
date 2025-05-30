use owo_colors::OwoColorize;

use crate::{misc::clap_mark, traits::StripAnsi};

// todo 编写统一的table组件
#[derive(Clone)]
pub struct Column {
    name: String,
    pub align: ColumnAlign,
    width: usize,
}

#[derive(Clone)]
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

        // 确保每行的单元格数量与列数一致
        for row in &rows {
            if row.cells.len() != columns.len() {
                panic!(
                    "{} Row cell count does not match column count",
                    clap_mark::fatal()
                );
            }
            for i in 0..columns.len() {
                columns[i].width = columns[i].width.max(row.cells[i].true_len());
            }
        }
        Self { columns, rows }
    }

    // todo 可以改成只要实现了to_table_row这个函数的都可以使用，即rows: Vec<T>
    pub fn display(columns: Vec<Column>, rows: Vec<TableRow>) {
        let table = Self::new(columns, rows);
        table.display_header();
        table.display_rows();
    }

    pub fn format_row(&self, cells: &[String]) -> String {
        let mut formatted: Vec<String> = vec![];
        for i in 0..self.columns.len() {
            let col = &self.columns[i];
            let cell = &cells[i];
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
