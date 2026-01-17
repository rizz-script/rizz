import { useEffect, useRef } from 'react';

interface OutputPanelProps {
  output: string[];
  isRunning: boolean;
}

export default function OutputPanel({ output, isRunning }: OutputPanelProps) {
  const outputRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (outputRef.current) {
      outputRef.current.scrollTop = outputRef.current.scrollHeight;
    }
  }, [output]);

  return (
    <div className="output-panel">
      <div className="output-header">
        <h3>Output</h3>
        {isRunning && <span className="status running">Running...</span>}
      </div>
      <div className="output-content" ref={outputRef}>
        {output.length === 0 ? (
          <div className="output-empty">
            Click "Run" to execute your code
          </div>
        ) : (
          output.map((line, index) => (
            <div
              key={index}
              className={`output-line ${
                line.startsWith('Error:') ? 'error' : ''
              }`}
            >
              {line}
            </div>
          ))
        )}
      </div>
    </div>
  );
}
