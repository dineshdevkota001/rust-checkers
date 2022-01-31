use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub enum CellValue {
    Black,
    Empty,
    Red,
    Blue,
    RedKing,
    BlueKing,
}

fn is_same_type_cellvalue(a: CellValue, b: CellValue) -> bool {
    if (a == b) {
        return true;
    }
    if (a == CellValue::Red && b == CellValue::RedKing)
        || (b == CellValue::Red && a == CellValue::RedKing)
        || (a == CellValue::Blue && b == CellValue::BlueKing)
        || (b == CellValue::Blue && a == CellValue::BlueKing)
    {
        return true;
    }
    return false;
}

pub fn is_same_player(a: CellValue, b: CellValue) -> bool {
    if a == CellValue::Black
        || b == CellValue::Black
        || a == CellValue::Empty
        || b == CellValue::Empty
    {
        return false;
    }
    return is_same_type_cellvalue(a, b);
}

#[derive(Copy, Clone)]
pub struct Cell {
    value: CellValue,
    is_selected: bool,
}

impl Cell {
    pub fn set_value(&mut self, value: CellValue) {
        self.value = value;
    }

    pub fn can_set_value(&self, value: CellValue) -> bool {
        if self.value == CellValue::Black {
            return false;
        }
        if self.value == CellValue::Empty {
            return true;
        }
        return false;
    }

    pub fn render_top_or_bottom(&self) -> String {
        match self.value {
            CellValue::Black => return String::from("#####"),
            _ => {
                if self.is_selected {
                    return String::from(" --- ");
                }
                return String::from("     ");
            }
        }
    }

    pub fn render_mid(&self) -> String {
        match self.value {
            CellValue::Black => return String::from("#####"),
            _ => {
                let cell_string = self.render_cell_value();
                if self.is_selected {
                    return format!("| {} |", cell_string);
                }
                return format!("  {}  ", cell_string);
            }
        }
    }
    pub fn render_cell_value(&self) -> String {
        match self.value {
            CellValue::Red => return String::from("x"),
            CellValue::RedKing => return String::from("X"),
            CellValue::Blue => return String::from("o"),
            CellValue::BlueKing => return String::from("O"),
            CellValue::Empty => return String::from(" "),
            _ => return String::from("#"),
        }
    }
    pub fn set_selected(&mut self) {
        self.is_selected = true;
    }
    pub fn set_deselected(&mut self) {
        self.is_selected = false;
    }
    pub fn get_value(&self) -> CellValue {
        return self.value;
    }

    pub fn new(value: CellValue) -> Cell {
        return Cell {
            value: value,
            is_selected: false,
        };
    }
}
