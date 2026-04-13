import React from 'react'

export type SlugEntry = {
  Content: React.ComponentType
  title?: string
  description?: string
}

export type PagesProps = {
  label: string
  slug?: string
  content?: any
  title?: string
  description?: string
  items?: PagesProps[]
}

export const createSlugMap = (items: PagesProps[]): Record<string, SlugEntry> =>
  Object.fromEntries(
    (function flatten(items): [string, SlugEntry][] {
      return items.flatMap((item) => [
        ...(item.slug && item.content
          ? [
              [
                item.slug,
                {
                  Content: item.content,
                  title: item?.title,
                  description: item?.description,
                },
              ] as [string, SlugEntry],
            ]
          : []),
        ...(item.items ? flatten(item.items) : []),
      ])
    })(items ?? []),
  )
