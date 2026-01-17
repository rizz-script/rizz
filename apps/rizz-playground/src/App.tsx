import { useState, useEffect } from 'react';
import Split from 'react-split';
import CodeEditor from './components/CodeEditor';
import OutputPanel from './components/OutputPanel';
import ControlBar from './components/ControlBar';
import { RizzScriptInterpreter, createInterpreter } from './runtime/interpreter';
import { EXAMPLES, DEFAULT_CODE } from './examples';
import './App.css';

function App() {
  const [code, setCode] = useState(DEFAULT_CODE);
  const [output, setOutput] = useState<string[]>([]);
  const [isRunning, setIsRunning] = useState(false);
  const [interpreter, setInterpreter] = useState<RizzScriptInterpreter | null>(null);
  const [wasmReady, setWasmReady] = useState(false);

  useEffect(() => {
    // Initialize interpreter (will try WASM first, fallback to JS)
    createInterpreter()
      .then((interp) => {
        setInterpreter(interp);
        setWasmReady(true);
      })
      .catch((error) => {
        console.warn('Failed to initialize interpreter:', error);
        // Create JS fallback interpreter
        const fallback = new RizzScriptInterpreter();
        setInterpreter(fallback);
        setWasmReady(true);
      });
  }, []);

  const handleRun = async () => {
    if (!interpreter) {
      setOutput(['Error: Interpreter not initialized']);
      return;
    }

    setIsRunning(true);
    setOutput([]);

    try {
      const result = await interpreter.execute(code);
      const allOutput = [
        ...result.stdout,
        ...result.stderr.map(e => `Error: ${e}`)
      ];
      setOutput(allOutput.length > 0 ? allOutput : ['Program executed successfully (no output)']);
    } catch (error: any) {
      setOutput([`Error: ${error.message}`]);
    } finally {
      setIsRunning(false);
    }
  };

  const handleClear = () => {
    setOutput([]);
  };

  const handleFormat = () => {
    // Basic formatting - add proper indentation
    const lines = code.split('\n');
    let indent = 0;
    const formatted = lines.map(line => {
      const trimmed = line.trim();
      if (trimmed.endsWith('{')) {
        const result = '  '.repeat(indent) + trimmed;
        indent++;
        return result;
      } else if (trimmed === '}') {
        indent = Math.max(0, indent - 1);
        return '  '.repeat(indent) + trimmed;
      } else {
        return '  '.repeat(indent) + trimmed;
      }
    });
    setCode(formatted.join('\n'));
  };

  const handleExampleChange = (exampleKey: string) => {
    if (EXAMPLES[exampleKey]) {
      setCode(EXAMPLES[exampleKey]);
      setOutput([]);
    }
  };

  return (
    <div className="playground">
      <header className="header">
        <div className="header-content">
          <h1>🔥 RizzScript Playground</h1>
          <p className="tagline">Write and run RizzScript code in your browser</p>
        </div>
      </header>

      <ControlBar
        onRun={handleRun}
        onClear={handleClear}
        onFormat={handleFormat}
        onExampleChange={handleExampleChange}
        isRunning={isRunning}
        wasmReady={wasmReady}
      />

      <div className="editor-container">
        <Split
          className="split"
          sizes={[60, 40]}
          minSize={300}
          gutterSize={8}
          direction="horizontal"
        >
          <div className="editor-pane">
            <div className="pane-header">
              <h3>Editor</h3>
              <span className="file-name">script.rizz</span>
            </div>
            <CodeEditor value={code} onChange={setCode} />
          </div>

          <OutputPanel output={output} isRunning={isRunning} />
        </Split>
      </div>

      <footer className="footer">
        <p>
          RizzScript v0.1.0 | 
          <a href="https://github.com/rizz-script/rizzz" target="_blank" rel="noopener noreferrer">
            GitHub
          </a> | 
          <a href="/docs" target="_blank" rel="noopener noreferrer">
            Documentation
          </a>
        </p>
      </footer>
    </div>
  );
}

export default App;
