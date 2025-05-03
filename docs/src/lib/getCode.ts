import fs from 'fs'
import path from 'path'
import { snakeCaseToCamelCase } from '@/lib/caseTransformations'

const getCodeFiles = (dir: string) => {
  const directories = fs.readdirSync(dir, { recursive: true }) as string[]
  return directories.filter((file) => ['.py', '.ts', '.rs', '.json', '.yaml'].includes(path.extname(file)))
}

const getCodeContent = (dir: string) => {
  return getCodeFiles(dir).map((file) => {
      const content = fs.readFileSync(path.join(dir, file), 'utf-8')
      const name =
        snakeCaseToCamelCase(path.basename(file, path.extname(file)).replace('.', '_')) +
        path.extname(file).replace('.', '').toUpperCase()
      return {
        [name]: content,
      }
    })
    .reduce((acc, next) => ({ ...acc, ...next }), {})
}

export const getCode = () => {
  return getCodeContent(path.join(process.cwd(), 'src', 'content', 'code'))
}
