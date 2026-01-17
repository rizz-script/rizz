// WASM Interpreter Integration
// This file will be used once the WASM module is ready

// Uncomment when WASM is built:
// import init, { RizzInterpreter, ExecutionResult } from '../wasm/rizz_wasm.js';

/*
let wasmInitialized = false;
let interpreter: RizzInterpreter | null = null;

export async function initWasm(): Promise<RizzInterpreter> {
  if (!wasmInitialized) {
    await init();
    interpreter = new RizzInterpreter();
    wasmInitialized = true;
    console.log('RizzScript WASM initialized');
  }
  return interpreter!;
}

export async function executeWasm(code: string): Promise<{
  stdout: string[];
  stderr: string[];
  exitCode: number;
}> {
  try {
    const interp = await initWasm();
    const result = interp.execute(code);
    
    return {
      stdout: result.output || [],
      stderr: result.error ? [result.error] : [],
      exitCode: result.success ? 0 : 1,
    };
  } catch (error: any) {
    return {
      stdout: [],
      stderr: [`WASM Error: ${error.message}`],
      exitCode: 1,
    };
  }
}

export async function checkSyntaxWasm(code: string): Promise<string | null> {
  try {
    const interp = await initWasm();
    // Use check_syntax function when available
    return null; // No syntax errors
  } catch (error: any) {
    return error.message;
  }
}
*/

// Temporary: Export the JS interpreter until WASM is ready
export { initWasm, RizzScriptInterpreter } from './interpreter';

export const WASM_AVAILABLE = false;

// Instructions for enabling WASM:
// 1. Build the WASM module: cd ../rizz-wasm && wasm-pack build --target web
// 2. Copy pkg/* to src/wasm/
// 3. Uncomment the code above
// 4. Set WASM_AVAILABLE = true
// 5. Update App.tsx to use executeWasm instead of the JS interpreter
