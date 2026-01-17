import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  srcDir: "contents",
  
  title: "RizzScript",
  description: "A utility-focused, esoteric scripting language with a vibe-based syntax. Optimized for async tasks, networking, and high-performance I/O.",
  
  head: [
    ['link', { rel: 'icon', href: '/favicon.ico' }]
  ],

  markdown: {
    // Configure code block highlighting
    // Since Shiki doesn't have RizzScript, we'll use JavaScript as a fallback
    // and apply custom CSS for better highlighting
    config: (md) => {
      // Add custom renderer for code blocks
      const defaultRender = md.renderer.rules.fence
      md.renderer.rules.fence = (tokens, idx, options, env, self) => {
        const token = tokens[idx]
        const info = token.info ? token.info.trim() : ''
        
        // If it's a rizz code block, use JavaScript highlighting as base
        // and wrap in custom div for styling
        if (info === 'rizz' || info.startsWith('rizz ')) {
          // Temporarily change to js for Shiki highlighting
          const originalInfo = token.info
          token.info = 'js'
          const result = defaultRender(tokens, idx, options, env, self)
          token.info = originalInfo // Restore original
          
          // Wrap in custom div with rizz class for CSS styling
          return `<div class="rizz-code-block" data-language="rizz">${result}</div>`
        }
        
        return defaultRender(tokens, idx, options, env, self)
      }
    }
  },

  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    logo: '/logo.svg',
    
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting Started', link: '/getting-started' },
      { text: 'Installation', link: '/installation' },
      { text: 'Syntax', link: '/syntax' },
      { text: 'Examples', link: '/examples' },
      { text: 'Spec', link: '/spec' },
      { text: 'LSP', link: '/lsp' },
      { text: 'VS Code', link: '/vscode-extension' }
    ],

    sidebar: [
      {
        text: 'Introduction',
        items: [
          { text: 'Getting Started', link: '/getting-started' },
          { text: 'Installation', link: '/installation' },
          { text: 'Features', link: '/features' }
        ]
      },
      {
        text: 'Language',
        items: [
          { text: 'Syntax & Keywords', link: '/syntax' },
          { text: 'Examples', link: '/examples' },
          { text: 'Language Specification', link: '/spec' }
        ]
      },
      {
        text: 'Tools',
        items: [
          { text: 'VS Code Extension', link: '/vscode-extension' },
          { text: 'Language Server Protocol', link: '/lsp' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/rizz-script/rizz' }
    ],

    search: {
      provider: 'local'
    },

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024 RizzScript Contributors'
    },

    editLink: {
      pattern: 'https://github.com/rizz-script/rizz/edit/main/docs/contents/:path',
      text: 'Edit this page on GitHub'
    },

    lastUpdated: {
      text: 'Last updated'
    }
  }
})
