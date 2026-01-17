// RizzScript WASM Interpreter
// Loads and uses the WASM module from /wasm/rizz_wasm.js

export interface RuntimeOutput {
  stdout: string[];
  stderr: string[];
  exitCode: number;
}

interface WasmModule {
  RizzInterpreter: new () => RizzInterpreter;
  ExecutionResult: {
    success: boolean;
    output: string[];
    error?: string;
  };
  init?: () => void; // Optional, may be called automatically
  default?: (module_or_path?: unknown) => Promise<unknown>; // Default export for initialization
}

interface RizzInterpreter {
  execute(code: string): ExecutionResult;
}

interface ExecutionResult {
  success: boolean;
  output: string[];
  error?: string;
}

let wasmModule: WasmModule | null = null;
let wasmInitialized = false;
let initPromise: Promise<void> | null = null;

/**
 * Initialize WASM module
 * This loads the WASM module from the public/wasm folder
 */
export async function initWasm(): Promise<void> {
  if (wasmInitialized) {
    return;
  }

  if (initPromise) {
    return initPromise;
  }

  initPromise = (async () => {
    try {
      // Load WASM module from public folder using dynamic import with URL
      // Files in /public are served as-is, so we need to construct the URL at runtime
      // This prevents Vite from trying to resolve it at build time
      const wasmUrl = new URL('/wasm/rizz_wasm.js', window.location.origin).href;
      
      // Use dynamic import with the full URL
      const imported = await import(/* @vite-ignore */wasmUrl);
      
      // Initialize WASM module via default export (required for wasm-bindgen)
      // The default export loads the WASM binary and initializes it
      if (imported.default) {
        await imported.default();
      }
      
      // Verify the module has the expected exports after initialization
      if (!imported || !imported.RizzInterpreter) {
        throw new Error('WASM module missing RizzInterpreter export');
      }
      
      wasmModule = imported as WasmModule;
      
      wasmInitialized = true;
      console.log('✅ RizzScript WASM initialized');
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.warn('⚠️ Failed to load WASM module:', errorMessage);
      console.warn('Falling back to JavaScript interpreter');
      wasmInitialized = false;
      wasmModule = null;
      throw new Error(errorMessage);
    }
  })();

  return initPromise;
}

/**
 * RizzScript Interpreter using WASM
 * Falls back to JS interpreter if WASM is not available
 */
export class RizzScriptInterpreter {
  private wasmInterpreter: RizzInterpreter | null = null;
  private useWasm: boolean = false;

  constructor() {
    // Will be initialized on first use
  }

  private async ensureInitialized() {
    if (!wasmInitialized && !initPromise) {
      try {
        await initWasm();
        this.useWasm = true;
        if (wasmModule) {
          this.wasmInterpreter = new wasmModule.RizzInterpreter();
        }
      } catch {
        // WASM failed, will use JS fallback
        this.useWasm = false;
      }
    } else if (wasmInitialized && wasmModule) {
      this.useWasm = true;
      if (!this.wasmInterpreter) {
        this.wasmInterpreter = new wasmModule.RizzInterpreter();
      }
    }
  }

  async execute(code: string): Promise<RuntimeOutput> {
    await this.ensureInitialized();

    if (this.useWasm && this.wasmInterpreter) {
      // Use WASM interpreter
      try {
        const result = this.wasmInterpreter.execute(code);
        return {
          stdout: result.output || [],
          stderr: result.error ? [result.error] : [],
          exitCode: result.success ? 0 : 1,
        };
      } catch (error: unknown) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        return {
          stdout: [],
          stderr: [`WASM Error: ${errorMessage}`],
          exitCode: 1,
        };
      }
    } else {
      // Fallback to JS interpreter
      return this.executeJS(code);
    }
  }

  private executeJS(code: string): RuntimeOutput {
    // Simple JS fallback interpreter
    const output: string[] = [];
    const errors: string[] = [];
    
    try {
      // Basic parsing and execution
      const lines = code.split('\n');
      
      for (const line of lines) {
        const trimmed = line.trim();
        if (!trimmed || trimmed.startsWith('//')) continue;
        
        // Handle Rizz() statements
        const rizzMatch = trimmed.match(/Rizz\s*\(\s*["']([^"']+)["']\s*\)/);
        if (rizzMatch) {
          output.push(rizzMatch[1]);
          continue;
        }
        
        // Handle variable declarations with Rizz
        const varRizzMatch = trimmed.match(/Rizz\s*\(\s*(\w+)\s*\)/);
        if (varRizzMatch) {
          // Simple variable lookup (very basic)
          output.push(varRizzMatch[1]);
          continue;
        }
        
        // Handle string concatenation in Rizz
        const concatMatch = trimmed.match(/Rizz\s*\(\s*["']([^"']+)["']\s*\+\s*["']([^"']+)["']\s*\)/);
        if (concatMatch) {
          output.push(concatMatch[1] + concatMatch[2]);
          continue;
        }
      }
      
      return {
        stdout: output,
        stderr: errors,
        exitCode: 0,
      };
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      return {
        stdout: output,
        stderr: [`Error: ${errorMessage}`],
        exitCode: 1,
      };
    }
  }
}

/**
 * Create a new interpreter instance
 */
export async function createInterpreter(): Promise<RizzScriptInterpreter> {
  const interpreter = new RizzScriptInterpreter();
  // Pre-initialize by calling execute with empty code
  // This will trigger initialization
  await interpreter.execute('');
  return interpreter;
}
