import { getDocsContent } from '../src/lib/mdx.ts'
import fs from 'fs'
import path from 'path'

const createSearchIndex = () => {
  const content = getDocsContent([]).map((element) => {
    if (element.slug === 'reference/changelog') return {
      slug: '/' + element.slug,
      label: 'Changelog',
      description: '',
      keywords: [],
    }

    return {
      slug: '/' + element.slug,
      label: element.metadata?.title,
      description: element.metadata?.description,
      keywords: element.metadata?.keywords,
    }
  })

  const filePath = path.join(process.cwd(), 'public', 'searchIndex.json')
  fs.writeFileSync(filePath, JSON.stringify(content, null, 2))
  console.log('Search index created')
}


createSearchIndex()