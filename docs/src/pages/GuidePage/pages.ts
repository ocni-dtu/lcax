import Calculate from './content/calculation/calculate.mdx'
import { frontmatter as calculateFrontmatter } from './content/calculation/calculate.mdx'
import Results from './content/calculation/results.mdx'
import { frontmatter as resultFrontmatter } from './content/calculation/results.mdx'
import CustomConverter from './content/conversion/custom.mdx'
import { frontmatter as customFrontmatter } from './content/conversion/custom.mdx'
import ILCD from './content/conversion/ilcd.mdx'
import { frontmatter as ilcdFrontmatter } from './content/conversion/ilcd.mdx'
import LCAByg from './content/conversion/lcabyg.mdx'
import { frontmatter as lcabygFrontmatter } from './content/conversion/lcabyg.mdx'
import RealTime from './content/conversion/realtimelca.mdx'
import { frontmatter as realTimeFrontmatter } from './content/conversion/realtimelca.mdx'
import DataStructure from './content/data-structure.mdx'
import { frontmatter as dataFrontmatter } from './content/data-structure.mdx'
import Installation from './content/installation.mdx'
import { frontmatter as installationFrontmatter } from './content/installation.mdx'
import Advanced from './content/validation/advanced.mdx'
import { frontmatter as advancedFrontmatter } from './content/validation/advanced.mdx'
import Validate from './content/validation/validate.mdx'
import { frontmatter as validateFrontmatter } from './content/validation/validate.mdx'

export const guideIndex = {
  label: 'Guides',
  items: [
    {
      label: 'Install LCAx',
      slug: '/guides/installation',
      content: Installation,
      title: installationFrontmatter.title,
      description: installationFrontmatter.description,
    },
    {
      label: 'Data Structure',
      slug: '/guides/data-structure',
      content: DataStructure,
      title: dataFrontmatter.title,
      description: dataFrontmatter.description,
    },
    {
      label: 'Conversion',
      items: [
        {
          label: 'ILCD',
          slug: '/guides/conversion/ilcd',
          content: ILCD,
          title: ilcdFrontmatter.title,
          description: ilcdFrontmatter.description,
        },
        {
          label: 'LCAByg',
          slug: '/guides/conversion/lcabyg',
          content: LCAByg,
          title: lcabygFrontmatter.title,
          description: lcabygFrontmatter.description,
        },
        {
          label: 'Real-Time LCA',
          slug: '/guides/conversion/realtimelca',
          content: RealTime,
          title: realTimeFrontmatter.title,
          description: realTimeFrontmatter.description,
        },
        {
          label: 'Custom Converter',
          slug: '/guides/conversion/custom',
          content: CustomConverter,
          title: customFrontmatter.title,
          description: customFrontmatter.description,
        },
      ],
    },
    {
      label: 'Validation',
      items: [
        {
          label: 'Validate',
          slug: '/guides/validation/validate',
          content: Validate,
          title: validateFrontmatter.title,
          description: validateFrontmatter.description,
        },
        {
          label: 'Advanced',
          slug: '/guides/validation/advanced',
          content: Advanced,
          title: advancedFrontmatter.title,
          description: advancedFrontmatter.description,
        },
      ],
    },
    {
      label: 'Calculation',
      items: [
        {
          label: 'Calculate',
          slug: '/guides/calculation/calculate',
          content: Calculate,
          title: calculateFrontmatter.title,
          description: calculateFrontmatter.description,
        },
        {
          label: 'Results',
          slug: '/guides/calculation/results',
          content: Results,
          title: resultFrontmatter.title,
          description: resultFrontmatter.description,
        },
      ],
    },
  ],
}
