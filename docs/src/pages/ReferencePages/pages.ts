import JavaScriptAPI from './content/api/javascript.mdx'
import PythonAPI from './content/api/python.md'
import { frontmatter as PythonFrontmatter } from './content/api/python.md'
import RustLCAxAPI from './content/api/rust/lcax.md'
import { frontmatter as RustLCAxFrontmatter } from './content/api/rust/lcax.md'
import RustLCAxCalcAPI from './content/api/rust/lcax_calculation.md'
import { frontmatter as RustLCAxCalcFrontmatter } from './content/api/rust/lcax_calculation.md'
import RustLCAxConvertAPI from './content/api/rust/lcax_convert.md'
import { frontmatter as RustLCAxConvertFrontmatter } from './content/api/rust/lcax_convert.md'
import RustLCAxCoreAPI from './content/api/rust/lcax_core.md'
import { frontmatter as RustLCAxCoreFrontmatter } from './content/api/rust/lcax_core.md'
import RustLCAxModelsAPI from './content/api/rust/lcax_models.md'
import { frontmatter as RustLCAxModelsFrontmatter } from './content/api/rust/lcax_models.md'
import RustLCAxValidationAPI from './content/api/rust/lcax_validation.md'
import { frontmatter as RustLCAxValidationFrontmatter } from './content/api/rust/lcax_validation.md'
import Changelog from './content/CHANGELOG.md'
import Migration from './content/migration.md'
import { frontmatter as migrationFrontmatter } from './content/migration.md'
import PythonSubclass from './content/python/subclass.mdx'
import { frontmatter as subclassFrontmatter } from './content/python/subclass.mdx'
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
      label: 'Python',
      items: [
        {
          label: 'Subclassing',
          slug: '/reference/python/subclass',
          content: PythonSubclass,
          title: subclassFrontmatter.title,
          description: subclassFrontmatter.description,
        },
      ],
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
        {
          label: 'Python',
          slug: '/reference/api/python',
          content: PythonAPI,
          title: PythonFrontmatter.title,
          description: PythonFrontmatter.description,
        },
        { label: 'TypeScript', slug: '/reference/api/typescript', content: JavaScriptAPI, title: 'TypeScript API' },
        {
          label: 'Rust',
          items: [
            {
              label: 'LCAx',
              slug: '/reference/api/rust/lcax',
              content: RustLCAxAPI,
              title: RustLCAxFrontmatter.title,
              description: RustLCAxFrontmatter.description,
            },
            {
              label: 'LCAx Calculation',
              slug: '/reference/api/rust/lcax_calculation',
              content: RustLCAxCalcAPI,
              title: RustLCAxCalcFrontmatter.title,
              description: RustLCAxCalcFrontmatter.description,
            },
            {
              label: 'LCAx Convert',
              slug: '/reference/api/rust/lcax_convert',
              content: RustLCAxConvertAPI,
              title: RustLCAxConvertFrontmatter.title,
              description: RustLCAxConvertFrontmatter.description,
            },
            {
              label: 'LCAx Core',
              slug: '/reference/api/rust/lcax_core',
              content: RustLCAxCoreAPI,
              title: RustLCAxCoreFrontmatter.title,
              description: RustLCAxCoreFrontmatter.description,
            },
            {
              label: 'LCAx Models',
              slug: '/reference/api/rust/lcax_models',
              content: RustLCAxModelsAPI,
              title: RustLCAxModelsFrontmatter.title,
              description: RustLCAxModelsFrontmatter.description,
            },
            {
              label: 'LCAx Validation',
              slug: '/reference/api/rust/lcax_validation',
              content: RustLCAxValidationAPI,
              title: RustLCAxValidationFrontmatter.title,
              description: RustLCAxValidationFrontmatter.description,
            },
          ],
        },
      ],
    },
  ],
}
