import JavaScriptAPI from './content/api/javascript.mdx'
import PythonAPI from './content/api/python.mdx'
import RustAPI from './content/api/rust.mdx'
import Changelog from './content/CHANGELOG.md'
import Migration from './content/migration.md'
import { frontmatter as migrationFrontmatter } from './content/migration.md'
import LCAxSchema from './content/schemas/lcax.mdx'
import { frontmatter as lcaxFrontmatter } from './content/schemas/lcax.mdx'
import ValidationSchema from './content/schemas/validation.mdx'
import { frontmatter as validationFrontmatter } from './content/schemas/validation.mdx'

export const referenceIndex = {
  label: 'Reference',
  items: [
    {
      label: 'Changelog',
      slug: '/reference/changelog',
      content: Changelog,
      title: 'Changelog',
      description: 'See what has changed in each version of LCAx',
    },
    {
      label: 'Migration',
      slug: '/reference/migration',
      content: Migration,
      title: migrationFrontmatter.title,
      description: migrationFrontmatter.description,
    },
    {
      label: 'JSON Schemas',
      items: [
        {
          label: 'LCAx',
          slug: '/reference/schemas/lcax',
          content: LCAxSchema,
          title: lcaxFrontmatter.title,
          description: lcaxFrontmatter.description,
        },
        {
          label: 'Validation',
          slug: '/reference/schemas/validation',
          content: ValidationSchema,
          title: validationFrontmatter.title,
          description: validationFrontmatter.description,
        },
      ],
    },
    {
      label: 'API',
      items: [
        { label: 'Python', slug: '/reference/api/python', content: PythonAPI, title: 'Python API' },
        { label: 'TypeScript', slug: '/reference/api/typescript', content: JavaScriptAPI, title: 'TypeScript API' },
        { label: 'Rust', slug: '/reference/api/rust', content: RustAPI, title: 'Rust API' },
      ],
    },
  ],
}
