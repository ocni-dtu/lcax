import { execSync } from 'node:child_process'
import { writeFileSync, readFileSync, mkdirSync, existsSync, rmSync } from 'node:fs'
import { join, resolve } from 'node:path'

const PROJECT_ROOT = resolve('..')
const API_DOCS_DIR = resolve('src/pages/ReferencePages/content/api')

async function main() {
  console.log('Generating API Documentation...')

  if (!existsSync(API_DOCS_DIR)) {
    mkdirSync(API_DOCS_DIR, { recursive: true })
  }

  // 1. Generate TypeScript Docs
  try {
    generateTSDocs()
  } catch (error) {
    console.error('Failed to generate TypeScript docs:', error)
  }

  // 2. Generate Python Docs
  try {
    generatePythonDocs()
  } catch (error) {
    console.error('Failed to generate Python docs:', error)
  }

  // 3. Generate Rust Docs
  try {
    generateRustDocs()
  } catch (error) {
    console.error('Failed to generate Rust docs:', error)
  }

  console.log('API documentation generation process completed.')
}

function generateTSDocs() {
  console.log('Generating TypeScript API documentation...')
  const tsInput = join(PROJECT_ROOT, 'packages/javascript/src/lcax.d.ts')
  const tempOut = join(PROJECT_ROOT, 'temp_ts_docs')

  if (existsSync(tempOut)) {
    rmSync(tempOut, { recursive: true, force: true })
  }

  const tempTsConfig = join(PROJECT_ROOT, 'temp_tsconfig.json')
  writeFileSync(
    tempTsConfig,
    JSON.stringify({
      compilerOptions: {
        target: 'ESNext',
        module: 'ESNext',
        moduleResolution: 'bundler',
        lib: ['ESNext', 'DOM'],
        skipLibCheck: true,
      },
      include: [tsInput],
    }),
  )

  // Use typedoc with markdown plugin
  execSync(
    `npx typedoc ${tsInput} --plugin typedoc-plugin-markdown --out ${tempOut} --hideBreadcrumbs true --hidePageTitle true --readme none --tsconfig ${tempTsConfig} --entryPointStrategy expand`,
    { stdio: 'inherit' },
  )

  // Post-process: Concatenate all files into a single one for simplicity in the docs site
  let combinedContent = ''

  function collectMarkdown(dir: string, relativeDir = '') {
    const items = execSync(`ls ${dir}`, { encoding: 'utf-8' }).split('\n').filter(Boolean)
    for (const item of items) {
      const fullPath = join(dir, item)
      if (execSync(`test -d ${fullPath} && echo "dir" || echo "file"`, { encoding: 'utf-8' }).trim() === 'dir') {
        collectMarkdown(fullPath, join(relativeDir, item))
      } else if (item.endsWith('.md')) {
        if (item === 'README.md') continue
        let fileContent = readFileSync(fullPath, 'utf-8')

        // Remove the navigation headers/footers added by TypeDoc
        fileContent = fileContent.replace(/\[\*\*lcax\*\*\]\(\.\.\/README\.md\) • \*\*Docs\*\*/gm, '')
        fileContent = fileContent.replace(/^\*\*lcax\*\* • \*\*Docs\*\*[\s\S]*?\*\*\*/gm, '')
        fileContent = fileContent.replace(/\*\*\*[\s\S]*?\*\*lcax\*\* • \*\*Docs\*\*/gm, '')

        combinedContent += `\n\n${fileContent}`
      }
    }
  }

  collectMarkdown(tempOut)

  if (combinedContent) {
    const mdxContent = wrapInMDX('TypeScript API Reference', 'LCAx TypeScript API Reference', combinedContent)
    writeFileSync(join(API_DOCS_DIR, 'javascript.mdx'), mdxContent)
  }

  rmSync(tempTsConfig)
  rmSync(tempOut, { recursive: true, force: true })
}

function generatePythonDocs() {
  console.log('Generating Python API documentation...')
  const content = execSync(`uvx griffe2md lcax`, { encoding: 'utf-8', cwd: PROJECT_ROOT })
  const mdxContent = wrapInMDX('Python API Reference', 'LCAx Python API Reference', content)
  writeFileSync(join(API_DOCS_DIR, 'python.md'), mdxContent)
}

function generateRustDocs() {
  console.log('Generating Rust API documentation...')
  execSync(`RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="-Z unstable-options --output-format json" cargo doc --no-deps`, {
    encoding: 'utf-8',
  })
  execSync(`cargo run --bin docs --features docs`)
}

function wrapInMDX(title: string, description: string, content: string) {
  return `---
title: ${title}
description: ${description}
---

${content}
`
}

main().catch(console.error)
