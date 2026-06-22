import DataStructure from './content/data-structure.mdx'
import { frontmatter as dataStructureFrontmatter } from './content/data-structure.mdx'
import Introduction from './content/introduction.mdx'
import { frontmatter as introductionFrontmatter } from './content/introduction.mdx'
import { frontmatter as pillarsFrontmatter } from './content/pillars.mdx'
import Pillars from './content/pillars.mdx'
import { frontmatter as rustFrontmatter } from './content/rust.mdx'
import Rust from './content/rust.mdx'

export const conceptIndex = {
  label: 'Concept',
  items: [
    {
      label: 'Introduction',
      slug: '/concept/introduction',
      content: Introduction,
      title: introductionFrontmatter.title,
      description: introductionFrontmatter.description,
    },
    {
      label: 'Why Rust',
      slug: '/concept/rust',
      content: Rust,
      title: rustFrontmatter.title,
      description: rustFrontmatter.description,
    },
    {
      label: "LCAx's Data Structure",
      slug: '/concept/data-structure',
      content: DataStructure,
      title: dataStructureFrontmatter.title,
      description: dataStructureFrontmatter.description,
    },
    {
      label: "LCAx's 4 Pillars",
      slug: '/concept/pillars',
      content: Pillars,
      title: pillarsFrontmatter.title,
      description: pillarsFrontmatter.description,
    },
  ],
}
