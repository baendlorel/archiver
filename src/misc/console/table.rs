use crate::traits::StripAnsi;

// todo 编写统一的table组件
#[derive(Clone)]
pub struct Column {
    pub name: String,
    pub align: ColumnAlign,
    pub min_width: usize,
}

#[derive(Clone)]
pub enum ColumnAlign {
    Left,
    Right,
    Center,
}

pub struct TableRow {
    pub cells: Vec<String>,
    pub widths: Vec<usize>,
}

impl TableRow {
    pub fn new(cells: Vec<String>) -> Self {
        let widths = cells.iter().map(|cell| cell.true_len()).collect();
        Self { cells, widths }
    }

    pub fn refresh_cell_widths(&mut self) {
        let widths = self.cells.iter().map(|cell| cell.true_len()).collect();
        self.widths = widths;
    }
}

pub trait TableDisplay {
    /// 定义表格的列结构
    fn columns() -> Vec<Column>;

    /// 将对象转换为表格行
    fn to_table_row(&self) -> TableRow;

    /// 显示多行数据的表格
    fn display_table(items: &[Self]) -> String
    where
        Self: Sized,
    {
        if items.is_empty() {
            return String::new();
        }

        let columns = Self::columns();
        let rows: Vec<TableRow> = items.iter().map(|item| item.to_table_row()).collect();

        // 计算每列的最大宽度
        let mut max_widths: Vec<usize> = columns.iter().map(|col| col.min_width).collect();

        for row in &rows {
            for (i, width) in row.widths.iter().enumerate() {
                if i < max_widths.len() {
                    max_widths[i] = max_widths[i].max(*width);
                }
            }
        }

        // 生成表格字符串
        let mut result = String::new();

        // 表头（可选）
        // result.push_str(&Self::format_header(&columns, &max_widths));
        // result.push('\n');

        // 数据行
        for row in rows {
            result.push_str(&Self::format_row(&row, &columns, &max_widths));
            result.push('\n');
        }

        result
    }

    /// 格式化单行数据
    fn format_row(row: &TableRow, columns: &[Column], max_widths: &[usize]) -> String {
        let mut formatted_cells = Vec::new();

        for (i, cell) in row.cells.iter().enumerate() {
            if i >= columns.len() || i >= max_widths.len() {
                break;
            }

            let col = &columns[i];
            let max_width = max_widths[i];
            let cell_width = cell.true_len();
            let padding = if max_width > cell_width {
                max_width - cell_width
            } else {
                0
            };

            let formatted_cell = match col.align {
                ColumnAlign::Left => format!("{}{}", cell, " ".repeat(padding)),
                ColumnAlign::Right => format!("{}{}", " ".repeat(padding), cell),
                ColumnAlign::Center => {
                    let left_pad = padding / 2;
                    let right_pad = padding - left_pad;
                    format!("{}{}{}", " ".repeat(left_pad), cell, " ".repeat(right_pad))
                }
            };

            formatted_cells.push(formatted_cell);
        }

        formatted_cells.join(" ")
    }
}
