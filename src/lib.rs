use wasm_bindgen::prelude::*;

use std::{collections::HashMap};

use calamine::{open_workbook_from_rs, Reader, Xlsx};
use serde::{Deserialize, Serialize};

// Our Add function
// wasm-pack requires "exported" functions
// to include #[wasm_bindgen]
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn return_string() -> String {
    "hello".into()
}

#[wasm_bindgen]
pub fn hello(name: &str) -> JsValue {
    JsValue::from_str(&format!("Hello from rust, {}!", name))
}

#[wasm_bindgen]
pub fn hello_str() -> String {
    // return "hello".to_string();
    // format!("Hello {}", "ddd")
    let mut tr = String::from("");
    for i in 0..10000 {
        tr.push_str("<tr>");
        tr.push_str(&format!("<td>{}</td>", i+1));
        for _ in 0..30{
            tr.push_str("<td>test</td>");
        }
        tr.push_str("</tr>");
    }
    return tr.to_string();
}

#[derive(Serialize, Deserialize, Debug)]
struct ExcelData{
    sheet_name: String,
    doc_list_id: u32,
    rows: Vec<HashMap<String,String>>
}

fn find_key_for_value(map: &HashMap<String, usize>, value: usize) -> Option<String> {
    map.iter().find_map(|(key, &val)| if val == value { Some(key.to_string()) } else { None })
}

#[wasm_bindgen(catch)]
pub fn get_data_from_excel(buffer: Vec<u8>, data: String) -> Result<String, JsValue> {

    let models: Vec<Model> = match serde_json::from_str(&data) 
    {
        Ok(result) => result,
        Err(error) => return Err(JsValue::from_str(&format!("Problem deserialize data: {:?}, error: {:?}", data, error))),
    };

    if models.last().is_none() {
        return Err(JsValue::from_str("The deserialize data is empty"));
    }

    let mut excel_data_array: Vec<ExcelData> = Vec::new();

    let mut excel: Xlsx<_> = match open_workbook_from_rs(std::io::Cursor::new(buffer)) {
        Ok(result) => result,
        Err(error) => return Err(JsValue::from_str(&format!("Can't open workbook from rs: {:?}", error))),
    };

    //let mut excel: Xlsx<_> = open_workbook(path).unwrap();
    for sheet_name in excel.sheet_names(){
        
        let model = models.iter().find(|m| m.sheet_name == sheet_name).unwrap();

        if model.sheet_name.is_empty() || model.keys.is_empty() {
            continue;
        }

        let mut column_codes: HashMap<String, usize> = HashMap::new();
        
        for (key, _) in &model.keys {
            column_codes.insert(key.to_string(), 0);
        }

        if column_codes.is_empty(){
            continue;
        }

        //excel_data_array.push(ExcelData { sheet_name: "g".to_string(), rows: Vec::new() });
        
        if let Ok(r) = excel.worksheet_range(&sheet_name){
            let mut rows: Vec<HashMap<String,String>> = Vec::new();
            let mut find_head_row = false;
            // println!("{}", r.rows().len());
            for row in r.rows() {

                let row_length = row.len();

                if find_head_row {
                    let mut row_data = HashMap::new();
                    for row_idx in 0..row_length {
                        if let Some(key) = find_key_for_value(&column_codes, row_idx){
                            row_data.insert(model.keys.get(&key).unwrap().to_string(), row[row_idx].to_string());
                        }
                    }

                    if !model.row_extension.is_empty(){
                        for (key, val) in &model.row_extension{
                            row_data.insert(key.to_string(), val.to_string());
                        }
                    }

                    rows.push(row_data);

                    continue;
                }

                if !find_head_row && row_length >= column_codes.len(){
                    let mut find_cell_count = 0;
                    for row_idx in 0..row_length {
                        if column_codes.contains_key(&row[row_idx].to_string()){
                            column_codes.insert(row[row_idx].to_string(), row_idx);
                            find_cell_count = find_cell_count + 1; 
                        }
                    }

                    find_head_row = find_cell_count == column_codes.len();
                }
            }

            excel_data_array.push(ExcelData { sheet_name, rows, doc_list_id: model.doc_list_id });
        }
    }

    match serde_json::to_string(&excel_data_array) {
        Ok(result)=>Ok(result),
        Err(error)=>Err(JsValue::from_str(&error.to_string()))
    }
    
}


#[derive(Serialize, Deserialize, Debug)]
struct Model{
    sheet_name: String,
    doc_list_id: u32,
    keys: HashMap<String,String>,
    row_extension: HashMap<String,String>
}

#[derive(Serialize, Deserialize, Debug)]
struct SheetDescription {
    sheet_name: String,
    tables: Vec<Tables>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tables {
    name: String,
    cells: Vec<Cell>,
    range: Range,
}

#[derive(Serialize, Deserialize, Debug)]
struct Range {
    start_row: u32,
    start_col: u32,
    end_row: u32,
    end_col: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cell {
    name: String,
    row: u32,
    col: u32,
}

fn set_range(range: &Range, cell: &Cell) -> Range {
    let start_row = if range.start_row <= cell.row { range.start_row } else { cell.row  };
    let start_col = if range.start_col <= cell.col { range.start_col } else { cell.col };
    let end_col = if range.end_col <= cell.col { cell.col } else { range.start_col };
    let end_row = if range.end_row <= cell.row { cell.row } else { range.start_row };

    return Range {
        start_row,
        start_col,
        end_col,
        end_row,
    };
}


#[wasm_bindgen(catch)]
pub fn get_data_model_from_excel(buffer: Vec<u8>) -> Result<String, JsValue> {

    let mut sheet_description_array: Vec<SheetDescription> = Vec::new();

    let mut excel: Xlsx<_> = match open_workbook_from_rs(std::io::Cursor::new(buffer)) {
        Ok(result) => result,
        Err(error) => return Err(JsValue::from_str(&format!("Can't open workbook from rs: {:?}", error))),
    };

    // let mut excel: Xlsx<_> = open_workbook(path).unwrap();
    for sheet_name in excel.sheet_names() {
        let mut sheet_description = SheetDescription {
            sheet_name,
            tables: Vec::<Tables>::new(),
        };

        if let Ok(r) = excel.worksheet_range(&sheet_description.sheet_name) {
            let mut tables = Vec::<Tables>::new();

            let mut row_index: u32 = 0;
            for row in r.rows() {
                row_index = row_index + 1;

                let row_length = row.len();

                for col_idx in 0..row_length {
                    if row[col_idx].to_string().contains("].[") {
                        let row_by_idx = row[col_idx].to_string();
                        let template_part: Vec<&str> = row_by_idx.split('.').collect();
                        if template_part.len() == 2 {
                            let table_name = template_part[0].replace("[", "").replace("]", "");
                            let col_name = template_part[1].replace("[", "").replace("]", "");

                            if let Some(t_find) = tables.iter_mut().find(|t| t.name == table_name) {
                                let cell = Cell {
                                    name: col_name,
                                    row: row_index,
                                    col: (col_idx + 1) as u32,
                                };

                                t_find.range = set_range(&t_find.range, &cell);
                                t_find.cells.push(cell);
                            } else {
                                let mut cells: Vec<Cell> = Vec::new();
                                cells.push(Cell {
                                    name: col_name,
                                    row: row_index,
                                    col: (col_idx + 1) as u32,
                                });
                                tables.push(Tables {
                                    name: table_name,
                                    cells,
                                    range: Range {
                                        start_row: row_index,
                                        start_col: (col_idx + 1) as u32,
                                        end_col: (col_idx + 1) as u32,
                                        end_row: row_index,
                                    },
                                });
                            }
                        }
                    }
                }
            }

            sheet_description.tables = tables;

            sheet_description_array.push(sheet_description);
        }
    }

    match serde_json::to_string(&sheet_description_array) {
        Ok(result)=>Ok(result),
        Err(error)=>Err(JsValue::from_str(&error.to_string()))
    }
    
}


#[wasm_bindgen]
pub fn test_struct(m: String) -> String {
    let mut test: String = String::from("");

    let model: Vec<Model> = serde_json::from_str(&m).unwrap();

    for m in model{
        test.push_str(&m.doc_list_id.to_string());
        test.push_str(&m.sheet_name);

        for (key, val) in m.keys{
            test.push_str(&key);
            test.push_str(&val);
        }
    }

    test.to_string()
}