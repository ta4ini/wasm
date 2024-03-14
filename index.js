// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init, { get_data_from_excel} from "./pkg/jsonfromxlsx.js";

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
        let res = get_data_from_excel(array, "1|2|3|4|5|6");
        console.log(res)
      };

      reader.onerror = function() {
        console.log(reader.error);
      };

      
   })
};

runWasm();