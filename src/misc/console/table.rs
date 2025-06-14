use owo_colors::OwoColorize;

use super::ansi;
use crate::misc::util::{int_partition, nsigma};
use crate::misc::{clap_mark, console::get_terminal_width};
use crate::traits::StripAnsi;

/// 最小的flex宽度，至少要能容纳省略号和一个字符
const MIN_FLEX_WIDTH: usize = 3;

const DEFAULT_MAX_FLEX_WIDTH: usize = 36;

const DEFAULT_COL_SPACE: usize = 1;

#[derive(Clone)]
pub struct Column {
    name: String,

    head_align: ColumnAlign,

    cell_align: ColumnAlign,

    /// 宽度，用来计算padding
    width: usize,

    /// 宽度的上限，在经过下方的策略进行计算后，予以修正
    /// - 就算计算得到的宽度大于这个值，也仍以此值为准
    max_width: usize,

    width_strategy: WidthStrategy,
}

/// 宽度策略，表示如何计算列宽
#[derive(Clone)]
enum WidthStrategy {
    /// 使用这一列所有cell里最宽的作为宽度
    Max,

    /// 使用`n-sigma`原则来消除明显太宽的宽度
    /// - 避免某一列过宽导致产生大量空白，导致空间利用率低、不美观
    NSigma,

    /// 取`Max`和`终端剩余宽度`中，较小的一个值
    /// - 如果有多列是这个配置，那么会平均分配宽度
    Flex,
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
    // 快速创建左对齐的列
    pub fn left(name: &str) -> Self {
        Self {
            name: name.to_string(),
            head_align: ColumnAlign::Left,
            cell_align: ColumnAlign::Left,
            width: name.true_len(),
            max_width: 0,
            width_strategy: WidthStrategy::Max,
        }
    }
    // 快速创建左对齐的列
    pub fn left_with_width(name: &str, width: usize) -> Self {
        Self {
            name: name.to_string(),
            head_align: ColumnAlign::Left,
            cell_align: ColumnAlign::Left,
            width,
            max_width: 0,
            width_strategy: WidthStrategy::Max,
        }
    }

    pub fn right(name: &str) -> Self {
        Self {
            name: name.to_string(),
            head_align: ColumnAlign::Left,
            cell_align: ColumnAlign::Right,
            width: name.true_len(),
            max_width: 0,
            width_strategy: WidthStrategy::Max,
        }
    }

    pub fn left_flex(name: &str) -> Self {
        Self {
            name: name.to_string(),
            head_align: ColumnAlign::Left,
            cell_align: ColumnAlign::Left,
            width: name.true_len(),
            max_width: 0,
            width_strategy: WidthStrategy::Flex,
        }
    }

    pub fn left_flex_with_max(name: &str, max_width: impl Into<Option<usize>>) -> Self {
        Self {
            name: name.to_string(),
            head_align: ColumnAlign::Left,
            cell_align: ColumnAlign::Left,
            width: name.true_len(),
            max_width: max_width.into().unwrap_or(DEFAULT_MAX_FLEX_WIDTH),
            width_strategy: WidthStrategy::Flex,
        }
    }

    pub fn left_nsigma(name: &str) -> Self {
        Self {
            name: name.to_string(),
            head_align: ColumnAlign::Left,
            cell_align: ColumnAlign::Left,
            width: name.true_len(),
            max_width: 0,
            width_strategy: WidthStrategy::NSigma,
        }
    }

    pub fn left_with_max(name: &str, max_width: usize) -> Self {
        Self {
            name: name.to_string(),
            head_align: ColumnAlign::Left,
            cell_align: ColumnAlign::Left,
            width: name.true_len(),
            max_width,
            width_strategy: WidthStrategy::Max,
        }
    }
}

impl TableRow {
    pub fn new(cells: Vec<String>) -> Self {
        Self { cells }
    }
}

impl Table {
    pub fn display<T>(rows: &Vec<T>)
    where
        T: Tablify,
    {
        let table_rows = rows
            .iter()
            .map(|r| r.to_table_row())
            .collect::<Vec<TableRow>>();
        let table = Self::new(T::get_table_columns(), table_rows);
        table.display_thead();
        table.display_tbody();
    }

    /// 用来竖排展示单独一条记录的时候用到的表头定义
    /// - 一般这个就够用了
    pub fn default_vertical_columns() -> Vec<Column> {
        vec![
            Column::left_with_width("Prop", 12),
            Column::left_flex_with_max("Value", None),
        ]
    }

    pub fn new(mut columns: Vec<Column>, rows: Vec<TableRow>) -> Self {
        // 确保列数不为0
        if columns.is_empty() {
            panic!("{} Columns vec cannot be empty", clap_mark::fatal());
        }

        // 列空隙默认为1
        let column_space = DEFAULT_COL_SPACE;

        // 获取所有宽度，每个元素代表这一列所有的宽度
        let mut width_matrix: Vec<Vec<usize>> = vec![];
        let field_count = columns.len();
        for i in 0..field_count {
            width_matrix.push(vec![columns[i].width]);
        }
        for row in &rows {
            for i in 0..field_count {
                width_matrix[i].push(row.cells[i].true_len());
            }
        }

        // 格式化每一列，不包括预设宽度是0的最后一列
        let mut flex_indexes: Vec<usize> = vec![];
        let mut fixed_width = 0;
        for i in 0..field_count {
            let raw_max = match columns[i].width_strategy {
                WidthStrategy::Max => *width_matrix[i].iter().max().unwrap(),
                WidthStrategy::NSigma => nsigma(&width_matrix[i]),
                WidthStrategy::Flex => {
                    columns[i].width = *width_matrix[i].iter().max().unwrap();
                    flex_indexes.push(i);
                    continue;
                }
            };

            // 这里只会有Max和NSigma的情况，Flex的已经被跳过
            // 不能超过上确界
            columns[i].width = if columns[i].max_width != 0 {
                raw_max.min(columns[i].max_width)
            } else {
                raw_max
            };

            fixed_width += columns[i].width;
        }

        // 没有flex的列，直接返回
        if flex_indexes.is_empty() {
            return Self {
                columns,
                rows,
                column_space,
            };
        }

        // 下面计算flex的情况
        let terminal_width = get_terminal_width();
        let least_width =
            fixed_width + (columns.len() - 1) * column_space + flex_indexes.len() * MIN_FLEX_WIDTH;

        // 不管宽度够不够了，超就超
        let normalized_width = terminal_width.max(least_width);
        let all_other_width = fixed_width + (columns.len() - flex_indexes.len()) * column_space;
        let flex_widths = int_partition(normalized_width - all_other_width, flex_indexes.len());

        for i in 0..flex_indexes.len() {
            let col = &mut columns[flex_indexes[i]];
            col.width = flex_widths[i].min(col.width);

            // 忽略掉最大宽度为0的情况，避免这一列直接没了
            if col.max_width != 0 {
                col.width = col.width.min(col.max_width);
            }
        }

        Self {
            columns,
            rows,
            column_space,
        }
    }

    pub fn format_row(&self, cells: &[String], is_head: bool) -> String {
        let mut formatted: Vec<String> = vec![];
        let mut linefeed: Vec<Vec<String>> = vec![];
        let mut max_lines = 1;

        // self.columns
        //     .iter()
        //     .for_each(|c| println!("{}:{}", c.name, c.width));
        // 把每一个单元格都按照宽度限制拆分为多行
        for i in 0..self.columns.len() {
            let col = &self.columns[i];
            let cell = &cells[i];
            // 宽度为0不参与padding
            if cell.true_len() <= col.width {
                let s = Table::pad_td(col, &cell, is_head);

                linefeed.push(vec![s]);
                continue;
            }

            // & 下面开始单元格内换行
            let chunks = ansi::chunkify(cell, col.width);
            let td: Vec<String> = chunks
                .iter()
                .map(|chunk| Table::pad_td(col, &chunk, is_head))
                .collect::<Vec<String>>();

            max_lines = max_lines.max(td.len());
            linefeed.push(td);
        }

        // 把多行的单元格拼接成真正的行
        for i in 0..max_lines {
            let mut row = vec![];
            for j in 0..self.columns.len() {
                if i < linefeed[j].len() {
                    row.push(linefeed[j][i].clone());
                } else {
                    // 如果这一行没有内容，就填充空白
                    row.push(" ".repeat(self.columns[j].width));
                }
            }
            formatted.push(row.join(&" ".repeat(self.column_space)));
        }
        formatted.join("\n")
    }

    pub fn display_thead(&self) {
        let th_list = self
            .columns
            .iter()
            .map(|col| col.name.clone())
            .collect::<Vec<String>>();
        println!("{}", &self.format_row(&th_list, true).bold().underline());
    }

    pub fn display_tbody(&self) {
        let mut tr_list: Vec<String> = vec![];
        for row in &self.rows {
            tr_list.push(self.format_row(&row.cells, false));
        }
        println!("{}", tr_list.join("\n"));
    }

    fn pad_td(col: &Column, chunk: &str, is_head: bool) -> String {
        let width = chunk.true_len();
        let padding = if col.width > width {
            col.width - width
        } else {
            // 真满格也不需要padding了
            return chunk.to_string();
        };

        let align = if is_head {
            &col.head_align
        } else {
            &col.cell_align
        };

        match align {
            ColumnAlign::Left => format!("{}{}", chunk, " ".repeat(padding)),
            ColumnAlign::Right => format!("{}{}", " ".repeat(padding), chunk),
            ColumnAlign::Center => {
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;
                format!("{}{}{}", " ".repeat(left_pad), chunk, " ".repeat(right_pad))
            }
        }
    }
}

pub trait Tablify {
    fn to_table_row(&self) -> TableRow;

    fn get_table_columns() -> Vec<Column>;

    fn display_vertically(&self);
}

#[macro_export]
macro_rules! kv_row {
    ($k:expr,$v:expr) => {
        crate::misc::console::table::TableRow::new(vec![$k.styled_field(), $v])
    };
}
