import fs from 'fs'
import path from 'path'
import { parse as parseYaml } from 'yaml'

export type Metadata = {
  title: string
  description: string
  keywords: string[]
}

const parseFrontmatter = (fileContent: string) => {
  const frontmatterRegex = /---\s*([\s\S]*?)\s*---/
  const match = frontmatterRegex.exec(fileContent)
  if (!match) {
    const content = fileContent.trim()
    return { metadata: null, content }
  }

  const content = fileContent.replace(frontmatterRegex, '').trim()
  const metadata = parseYaml(match[1])

  return { metadata: metadata as Metadata, content }
}

const getMDXFiles = (dir: string) => {
  const directories = fs.readdirSync(dir, { recursive: true }) as string[]
  return directories.filter((file) => ['.mdx', '.md'].includes(path.extname(file)))
}

const readMDXFile = (filePath: string) => {
  const rawContent = fs.readFileSync(filePath, 'utf-8')
  return parseFrontmatter(rawContent)
}

const getMDXData = (dir: string, prefix: string) => {
  return getMDXFiles(dir).map((file) => {
    const { metadata, content } = readMDXFile(path.join(dir, file))
    const slug = `${prefix}/${file
      .replace(/\.md(x)?/, '')
      .replace(/\\/g, '/')
      .toLowerCase()
      .trim()}`

    return {
      metadata,
      slug,
      content,
    }
  })
}

export const getDocsContent = (folder: string[] | undefined = []) => {
  const contentMap: Record<string, string> = {
    concept: path.join(process.cwd(), 'src', 'pages', 'ConceptPages', 'content'),
    guides: path.join(process.cwd(), 'src', 'pages', 'GuidePage', 'content'),
    reference: path.join(process.cwd(), 'src', 'pages', 'ReferencePages', 'content'),
  }

  if (folder && folder.length > 0) {
    const key = folder[0]
    if (contentMap[key]) {
      return getMDXData(contentMap[key], key)
    }
  }

  return Object.entries(contentMap).flatMap(([key, dir]) => getMDXData(dir, key))
}