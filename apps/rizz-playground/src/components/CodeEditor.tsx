import Editor from '@monaco-editor/react';
import { editor, languages } from 'monaco-editor';
import * as monaco from 'monaco-editor';
import { useRef } from 'react';

interface CodeEditorProps {
  value: string;
  onChange: (value: string) => void;
}

// RizzScript syntax highlighting definition
const rizzLanguageDefinition: languages.IMonarchLanguage = {
  defaultToken: 'text',
  
  tokenizer: {
    root: [
      // Comments - must come first
      [/\/\/.*$/, 'comment'],
      [/\/\*/, 'comment', '@comment'],
      
      // Strings - must come before keywords to avoid matching keywords inside strings
      [/r"/, 'string.regexp', '@string_regexp'],
      [/"([^"\\]|\\.)*$/, 'string.invalid'],
      [/'([^'\\]|\\.)*$/, 'string.invalid'],
      [/"/, 'string', '@string_double'],
      [/'/, 'string', '@string_single'],
      
      // Numbers - must come before identifiers
      [/\d+\.\d+/, 'number.float'],
      [/\d+/, 'number'],
      
      // Operators - check before keywords to catch operators
      [/==|!=|<=|>=|<|>/, 'keyword.operator.comparison'],
      [/&&|\|\||!/, 'keyword.operator.logical'],
      [/\+|-|\*|\/|%/, 'keyword.operator.arithmetic'],
      [/\.\./, 'keyword.operator.range'],
      [/=/, 'keyword.operator.assignment'],
      
      // Keywords - Control flow (must come before identifiers)
      [/\b(Maybe|Unless|Crazy|Attempt|Eww|HawkTuah|Vibe|Chill)\b/, 'keyword.control'],
      [/\b(if|else|for|while|try|catch|finally|async|await)\b/, 'keyword.control.js'],
      
      // Storage types
      [/\b(Ayo|Yoo|let|const)\b/, 'storage.type'],
      
      // Other keywords
      [/\b(Bruh|function)\b/, 'keyword.other'],
      [/\b(Rizz|return|Cringe|throw)\b/, 'keyword.control.flow'],
      
      // Import/Export
      [/\b(Bring|import|export|from)\b/, 'keyword.import'],
      
      // Constants
      [/\b(true|false|null)\b/, 'constant.language'],
      
      // Builtin functions - HTTP
      [/\b(Spit|Yeet|Flex|Ghost)\b/, 'support.function.builtin.http'],
      
      // Builtin functions - TCP
      [/\b(Listen|Holla|Peek|Whisper|Dip)\b/, 'support.function.builtin.tcp'],
      
      // Builtin functions - IO
      [/\b(Snag|Stash|KeepAdding|Trash|FileExists)\b/, 'support.function.builtin.io'],
      
      // Builtin functions - JSON
      [/\b(Decode|Encode)\b/, 'support.function.builtin.json'],
      
      // Builtin functions - Regex
      [/\b(Hunt|Swap|Matches|Split)\b/, 'support.function.builtin.regex'],
      
      // Identifiers and functions (must come last, after all keywords)
      [/\b[a-zA-Z_][a-zA-Z0-9_]*\s*(?=\()/, 'entity.name.function'],
      [/\b[a-zA-Z_][a-zA-Z0-9_]*/, 'identifier'],
      
      // Whitespace
      { include: '@whitespace' },
    ],
    
    comment: [
      [/[^/*]+/, 'comment'],
      [/\/\*/, 'comment', '@push'],
      [/\*\//, 'comment', '@pop'],
      [/[/*]/, 'comment'],
    ],
    
    string_double: [
      [/[^\\"]+/, 'string'],
      [/\\./, 'constant.character.escape'],
      [/"/, 'string', '@pop'],
    ],
    
    string_single: [
      [/[^\\']+/, 'string'],
      [/\\./, 'constant.character.escape'],
      [/'/, 'string', '@pop'],
    ],
    
    string_regexp: [
      [/[^\\"]+/, 'string.regexp'],
      [/\\./, 'constant.character.escape.regexp'],
      [/"/, 'string.regexp', '@pop'],
    ],
    
    whitespace: [
      [/[ \t\r\n]+/, 'white'],
    ],
  },
};

// Language configuration
const rizzLanguageConfiguration: languages.LanguageConfiguration = {
  comments: {
    lineComment: '//',
    blockComment: ['/*', '*/'],
  },
  brackets: [
    ['{', '}'],
    ['[', ']'],
    ['(', ')'],
  ],
  autoClosingPairs: [
    { open: '{', close: '}' },
    { open: '[', close: ']' },
    { open: '(', close: ')' },
    { open: '"', close: '"', notIn: ['string'] },
    { open: "'", close: "'", notIn: ['string', 'comment'] },
  ],
  surroundingPairs: [
    { open: '{', close: '}' },
    { open: '[', close: ']' },
    { open: '(', close: ')' },
    { open: '"', close: '"' },
    { open: "'", close: "'" },
  ],
  folding: {
    markers: {
      start: /^\s*\/\/\s*#?region\b/,
      end: /^\s*\/\/\s*#?endregion\b/,
    },
  },
  wordPattern: /(-?\d*\.\d\w*)|([^`~!@#%^&*()[\]{}\\|;:'",.<>/?\s-+=]+)/,
  indentationRules: {
    increaseIndentPattern: /^((?!\/\/).)*(\{[^}"'`]*|\([^)"'`]*|\[[^\]]"'`]*)$/,
    decreaseIndentPattern: /^((?!.*?\/\*).*\*\/)?\s*[}\]]/,
  },
};

// Snippets for code completion (range will be added dynamically in provideCompletionItems)
const rizzSnippets = [
  {
    label: 'ayo',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Ayo ${1:name} = ${2:value}',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Declare a mutable variable',
  },
  {
    label: 'yoo',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Yoo ${1:NAME} = ${2:value}',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Declare an immutable constant',
  },
  {
    label: 'bruh',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Bruh ${1:name}(${2:params}) {\n\t${3:// body}\n\tRizz(${4:result})\n}',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Define a function',
  },
  {
    label: 'bruhAsync',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Bruh ${1:name}(${2:params}) HawkTuah {\n\t${3:// async body}\n\tRizz(${4:result})\n}',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Define an async function',
  },
  {
    label: 'maybe',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Maybe ${1:condition} {\n\t${2:// body}\n}',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'If conditional',
  },
  {
    label: 'maybeUnless',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Maybe ${1:condition} {\n\t${2:// if body}\n} Unless {\n\t${3:// else body}\n}',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'If-else conditional',
  },
  {
    label: 'crazy',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Crazy ${1:i} in ${2:0}..${3:10} {\n\t${4:// body}\n}',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'For loop with range',
  },
  {
    label: 'crazyIn',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Crazy ${1:item} in ${2:array} {\n\t${3:// body}\n}',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'For loop iterating array',
  },
  {
    label: 'attempt',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Attempt {\n\t${1:// try body}\n} Eww (${2:error}) {\n\t${3:// catch body}\n}',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Try-catch error handling',
  },
  {
    label: 'spit',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Ayo ${1:response} = Spit("${2:url}")',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'HTTP GET request',
  },
  {
    label: 'yeet',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Ayo ${1:response} = Yeet("${2:url}", ${3:data})',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'HTTP POST request',
  },
  {
    label: 'flex',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Ayo ${1:response} = Flex("${2:url}", ${3:data})',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'HTTP PUT request',
  },
  {
    label: 'ghost',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Ghost("${1:url}")',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'HTTP DELETE request',
  },
  {
    label: 'snag',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Ayo ${1:content} = Snag("${2:filename}")',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Read file contents',
  },
  {
    label: 'stash',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Stash("${1:filename}", ${2:content})',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Write to file',
  },
  {
    label: 'listen',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Bruh ${1:handleRequest}(${2:socket}) HawkTuah {\n\tAyo message = Peek($2)\n\t${3:// handle request}\n\tWhisper($2, ${4:response})\n\tDip($2)\n}\n\nAyo server = Listen(${5:8080}, $1)',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Create TCP server',
  },
  {
    label: 'decode',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Ayo ${1:obj} = Decode(${2:jsonString})',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Parse JSON string',
  },
  {
    label: 'encode',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Ayo ${1:json} = Encode(${2:obj})',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Convert object to JSON',
  },
  {
    label: 'vibe',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Ayo ${1:task} = Vibe ${2:asyncFunction}(${3:args})',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Spawn async task',
  },
  {
    label: 'chill',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Ayo ${1:result} = Chill(${2:task})',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Await task completion',
  },
  {
    label: 'hunt',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Ayo ${1:matches} = Hunt(${2:text}, r"${3:pattern}")',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Find regex matches',
  },
  {
    label: 'rizz',
    kind: languages.CompletionItemKind.Snippet,
    insertText: 'Rizz(${1:value})',
    insertTextRules: languages.CompletionItemInsertTextRule.InsertAsSnippet,
    documentation: 'Return value or print',
  },
];

// Register RizzScript language using Monaco instance
function registerRizzLanguage(monacoInstance: typeof monaco) {
  // Check if already registered
  const existingLang = monacoInstance.languages.getLanguages().find(lang => lang.id === 'rizz');
  if (existingLang) {
    console.log('RizzScript language already registered');
    return; // Already registered
  }
  
  console.log('Registering RizzScript language...');
  
  // Register the language
  monacoInstance.languages.register({ id: 'rizz' });
  
  // Set Monarch tokenizer
  monacoInstance.languages.setMonarchTokensProvider('rizz', rizzLanguageDefinition);
  console.log('Monarch tokenizer set');
  
  // Set language configuration
  monacoInstance.languages.setLanguageConfiguration('rizz', rizzLanguageConfiguration);
  console.log('Language configuration set');
  
  // Define theme rules for syntax highlighting
  // This ensures tokens get colored properly
  monacoInstance.editor.defineTheme('rizz-dark', {
    base: 'vs-dark',
    inherit: true,
    rules: [
      { token: 'comment', foreground: '6A9955', fontStyle: 'italic' },
      { token: 'string', foreground: 'CE9178' },
      { token: 'string.regexp', foreground: 'D16969' },
      { token: 'string.invalid', foreground: 'f48771' },
      { token: 'number', foreground: 'B5CEA8' },
      { token: 'number.float', foreground: 'B5CEA8' },
      { token: 'keyword.control', foreground: 'C586C0', fontStyle: 'bold' },
      { token: 'keyword.control.js', foreground: 'C586C0' },
      { token: 'keyword.control.flow', foreground: 'C586C0', fontStyle: 'bold' },
      { token: 'keyword.other', foreground: 'C586C0' },
      { token: 'keyword.import', foreground: 'C586C0' },
      { token: 'storage.type', foreground: '569CD6', fontStyle: 'bold' },
      { token: 'support.function.builtin.http', foreground: 'DCDCAA' },
      { token: 'support.function.builtin.tcp', foreground: 'DCDCAA' },
      { token: 'support.function.builtin.io', foreground: 'DCDCAA' },
      { token: 'support.function.builtin.json', foreground: 'DCDCAA' },
      { token: 'support.function.builtin.regex', foreground: 'DCDCAA' },
      { token: 'entity.name.function', foreground: 'DCDCAA' },
      { token: 'constant.language', foreground: '569CD6' },
      { token: 'constant.character.escape', foreground: 'D7BA7D' },
      { token: 'constant.character.escape.regexp', foreground: 'D7BA7D' },
      { token: 'keyword.operator', foreground: 'D4D4D4' },
      { token: 'keyword.operator.comparison', foreground: 'D4D4D4' },
      { token: 'keyword.operator.logical', foreground: 'D4D4D4' },
      { token: 'keyword.operator.arithmetic', foreground: 'D4D4D4' },
      { token: 'keyword.operator.assignment', foreground: 'D4D4D4' },
      { token: 'keyword.operator.range', foreground: 'D4D4D4' },
      { token: 'identifier', foreground: '9CDCFE' },
      { token: 'text', foreground: 'D4D4D4' },
      { token: 'white', foreground: 'D4D4D4' },
    ],
    colors: {},
  });
  console.log('Theme rizz-dark defined');
  
  // Register completion provider with snippets
  monacoInstance.languages.registerCompletionItemProvider('rizz', {
    provideCompletionItems: (model, position) => {
      const word = model.getWordUntilPosition(position);
      const range = {
        startLineNumber: position.lineNumber,
        endLineNumber: position.lineNumber,
        startColumn: word.startColumn,
        endColumn: word.endColumn,
      };
      
      // Add range to each snippet
      const suggestions: languages.CompletionItem[] = rizzSnippets.map((snippet): languages.CompletionItem => ({
        ...snippet,
        range,
      }));
      
      return {
        suggestions,
      };
    },
    triggerCharacters: ['a', 'y', 'b', 'm', 'c', 's', 'Y', 'd', 'e', 'v', 'h', 'r'],
  });
  
  console.log('✅ RizzScript language registration complete');
}

export default function CodeEditor({ value, onChange }: CodeEditorProps) {
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null);

  const handleEditorChange = (value: string | undefined) => {
    onChange(value || '');
  };

  const handleBeforeMount = (monacoInstance: typeof monaco) => {
    // Register language before editor is created
    registerRizzLanguage(monacoInstance);
  };

  const handleEditorDidMount = (editorInstance: editor.IStandaloneCodeEditor, monacoInstance: typeof monaco) => {
    editorRef.current = editorInstance;
    
    
    // Ensure language is registered
    registerRizzLanguage(monacoInstance);
    
    // Set the language explicitly to ensure it's applied
    const model = editorInstance.getModel();
    if (model) {
      const currentLang = model.getLanguageId();
      
      if (currentLang !== 'rizz') {
        monacoInstance.editor.setModelLanguage(model, 'rizz');
      }
    }
    
    // Focus editor
    editorInstance.focus();
  };

  return (
    <Editor
      height="100%"
      defaultLanguage="rizz"
      theme="rizz-dark"
      value={value}
      onChange={handleEditorChange}
      beforeMount={handleBeforeMount}
      onMount={handleEditorDidMount}
      options={{
        minimap: { enabled: false },
        fontSize: 14,
        lineNumbers: 'on',
        roundedSelection: false,
        scrollBeyondLastLine: false,
        automaticLayout: true,
        tabSize: 2,
        wordWrap: 'on',
        suggestOnTriggerCharacters: true,
        quickSuggestions: {
          other: true,
          comments: false,
          strings: true,
        },
        acceptSuggestionOnEnter: 'on',
        tabCompletion: 'on',
        wordBasedSuggestions: 'off',
        parameterHints: { enabled: true },
        formatOnPaste: true,
        formatOnType: true,
        autoIndent: 'full',
        bracketPairColorization: { enabled: true },
        colorDecorators: true,
      }}
    />
  );
}
