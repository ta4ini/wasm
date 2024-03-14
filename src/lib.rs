// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

use calamine::{open_workbook, open_workbook_from_rs, Reader, Xlsx};
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
    rows: Vec<HashMap<String,String>>
}

fn find_key_for_value(map: &HashMap<String, usize>, value: usize) -> Option<String> {
    map.iter().find_map(|(key, &val)| if val == value { Some(key.to_string()) } else { None })
}

#[wasm_bindgen]
pub fn get_data_from_excel(buffer: Vec<u8>, column_str: &str) -> String {

    // if path.is_empty() || column_str.is_empty() {
    //     return String::new();
    // }

    let mut column_codes: HashMap<String, usize> = HashMap::new();
    column_str.split('|').for_each(|code| {
        column_codes.insert(code.to_string(), 0);
    });

    if column_codes.is_empty(){
        return String::new();
    }

    let mut excel_data_array: Vec<ExcelData> = Vec::new();


    // let string = "foo";
    // println!("{:?}", string.as_bytes().to_vec());

    // let f = File::open("test.xlsx").unwrap();
    // let mut reader = BufReader::new(f);
    // let mut buffer = Vec::new();

    // // Read file into vector.
    // reader.read_to_end(&mut buffer).unwrap();
    let mut excel: Xlsx<_> = open_workbook_from_rs(std::io::Cursor::new(buffer)).unwrap();

    //let mut excel: Xlsx<_> = open_workbook(path).unwrap();
    for sheet_name in excel.sheet_names(){
        
        println!("{}", sheet_name);
        
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
                            row_data.insert(key, row[row_idx].to_string());
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

            excel_data_array.push(ExcelData { sheet_name, rows });
        }
    }

    serde_json::to_string(&excel_data_array).unwrap()
}