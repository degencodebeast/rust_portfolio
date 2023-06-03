mod using_enums {
    pub enum SpreadsheetCell<'a> {
        Int(i32),
        Float(f64),
        Text(&'a str),
    }

    const ROW: [SpreadsheetCell; 3] = [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text("blue"),
        SpreadsheetCell::Float(10.12),
    ];
}