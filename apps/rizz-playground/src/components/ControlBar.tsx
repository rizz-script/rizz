interface ControlBarProps {
  onRun: () => void;
  onClear: () => void;
  onFormat: () => void;
  onExampleChange: (example: string) => void;
  isRunning: boolean;
  wasmReady?: boolean;
}

const EXAMPLES = [
  { name: 'Hello World', key: 'hello' },
  { name: 'Variables & Types', key: 'variables' },
  { name: 'Functions', key: 'functions' },
  { name: 'Async Example', key: 'async' },
  { name: 'Arrays & Objects', key: 'collections' },
];

export default function ControlBar({
  onRun,
  onClear,
  onFormat,
  onExampleChange,
  isRunning,
  wasmReady = false,
}: ControlBarProps) {
  return (
    <div className="control-bar">
      <div className="control-group">
        <button
          className="btn btn-primary"
          onClick={onRun}
          disabled={isRunning || !wasmReady}
        >
          ▶ Run
        </button>
        <button className="btn" onClick={onClear}>
          Clear Output
        </button>
        <button className="btn" onClick={onFormat}>
          Format Code
        </button>
        {wasmReady && (
          <span className="status-indicator" title="Interpreter ready">
            ✓ Ready
          </span>
        )}
        {!wasmReady && (
          <span className="status-indicator loading" title="Initializing interpreter...">
            ⏳ Loading...
          </span>
        )}
      </div>

      <div className="control-group">
        <label htmlFor="examples">Examples:</label>
        <select
          id="examples"
          className="example-select"
          onChange={(e) => onExampleChange(e.target.value)}
          defaultValue=""
        >
          <option value="" disabled>
            Choose an example...
          </option>
          {EXAMPLES.map((example) => (
            <option key={example.key} value={example.key}>
              {example.name}
            </option>
          ))}
        </select>
      </div>
    </div>
  );
}
