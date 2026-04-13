import { conceptIndex } from './ConceptPages'
import { guideIndex } from './GuidePage'
import { referenceIndex } from './ReferencePages'

export interface PagesProps {
  label: string
  slug?: string
  content?: any
  items?: PagesProps[]
}

export const pages: PagesProps[] = [conceptIndex, guideIndex, referenceIndex]
