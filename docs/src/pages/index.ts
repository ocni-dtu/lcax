import { conceptIndex } from './ConceptPages/pages'
import { guideIndex } from './GuidePage/pages'
import { referenceIndex } from './ReferencePages/pages'

export interface PagesProps {
  label: string
  slug?: string
  content?: any
  items?: PagesProps[]
}

export const pages: PagesProps[] = [conceptIndex, guideIndex, referenceIndex]
