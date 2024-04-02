// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init, { get_data_from_excel, get_data_model_from_excel } from "./pkg/jsonfromxlsx.js";

const runWasm = async () => {
    await init()

  // Instantiate our wasm module
//   const helloWorld = await init("./pkg/test3_bg.wasm");

  // Call the Add function export from wasm, save the result
//   const addResult = helloWorld.add(24, 24);

  // Set the result onto the body
   //document.body.querySelector('.table_body').innerHTML = hello_str()

   const input = document.querySelector(".excel_sel");
   const button = document.querySelector('.submit_btn');
   button.addEventListener('click', (e)=>{
      let struct = JSON.stringify([
        {
        sheet_name: "Лист 1",
        doc_list_id: 305,
        keys: {'1':'1','2':'2','3':'3','4':'4','5':'5','6':'6','7':'8'}, //key = column number, val = key in model
        row_extension: {'7':''} //empty data for row
      }])

      //console.log(test_struct(struct))

      let file = input.files[0]
      // console.log(file.webkitRelativePath);
      let reader = new FileReader();

      if (file) {
        reader.readAsArrayBuffer(file);
      }

      reader.onload = function() {
        // console.log(reader.result);
        let array = new Uint8Array(reader.result),
        binaryString = String.fromCharCode.apply(null, array);

        // console.log(binaryString)
        try{
          const start = performance.now();
          //let res = get_data_from_excel(array, struct);
          console.log(get_data_model_from_excel(array))
          const end = performance.now();
          console.log(`Execution time: ${end - start} ms`);
        }
        catch(e){
          console.error(e)
        }
      };

      reader.onerror = function() {
        console.log(reader.error);
      };

      
   })
};

runWasm();