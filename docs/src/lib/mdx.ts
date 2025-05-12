import fs from 'fs'
import path from 'path'
import { default as matter } from 'gray-matter'

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
  const metadata = matter(fileContent).data

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

const getMDXData = (dir: string) => {
  return getMDXFiles(dir).map((file) => {
    const { metadata, content } = readMDXFile(path.join(dir, file))
    const slug = file
      .replace(/\.md(x)?/, '')
      .toLowerCase()
      .trim()

    return {
      metadata,
      slug,
      content,
    }
  })
}

export const getDocsContent = (folder: string[] | undefined = []) => {
  return getMDXData(path.join(process.cwd(), 'src', 'content', ...folder))
}
