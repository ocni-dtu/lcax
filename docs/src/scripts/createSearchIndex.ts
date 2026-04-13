import fs from 'fs'
import path from 'path'

import { getDocsContent } from '@/lib/mdx'

const createSearchIndex = () => {
  const content = getDocsContent([]).map((element) => {
    if (element.slug === 'reference/changelog')
      return {
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

  const filePath = path.join(process.cwd(), 'src', 'assets', 'searchIndex.json')
  fs.writeFileSync(filePath, JSON.stringify(content, null, 2))
  console.log('Search index created')
}

createSearchIndex()
