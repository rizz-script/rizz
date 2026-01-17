// https://vitepress.dev/guide/custom-theme
import { h } from 'vue'
import type { Theme } from 'vitepress'
import DefaultTheme from 'vitepress/theme'
import './style.css'
import './rizz-syntax.css'

export default {
  extends: DefaultTheme,
  Layout: () => {
    return h(DefaultTheme.Layout, null, {
      // https://vitepress.dev/guide/extending-default-theme#layout-slots
      'home-hero-image': () => h('div', {
        class: 'rizz-hero-image',
        style: {
          background: 'linear-gradient(135deg, #8b5cf6 0%, #6366f1 50%, #ec4899 100%)',
          borderRadius: '50%',
          width: '200px',
          height: '200px',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          fontSize: '4rem',
          fontWeight: 'bold',
          color: 'white',
          boxShadow: '0 20px 60px rgba(139, 92, 246, 0.4)',
          margin: '0 auto',
          animation: 'pulse 2s ease-in-out infinite'
        }
      }, 'R')
    })
  },
  enhanceApp({ app, router, siteData }) {
    // Add custom enhancements here if needed
  }
} satisfies Theme
