# LCAx Compare Design System

Visual language distilled from `@lcax/compare`. Use this as the source of truth when restyling other LCAx apps or adding screens here.

The product feels **editorial and architectural**: large type, full-viewport sections, a pale grey canvas, mustard-yellow accents, and photography as identity. Chrome is almost absent. Data (charts, project facts) is treated as typography, not dashboard widgets.

Implementation lives in:

- Shared base: `packages/ui/src/components/Theme/`
- Compare overrides: `apps/compare/src/components/Theme/`
- Layout and patterns: `AppLayout`, landing/projects/details pages, and section components

Compare merges `@lcax/ui`’s Mantine theme, then overrides color, radius, heading weight, black, and container sizes.

---

## Principles

1. **Space first.** Prefer `100vh` (or `mih='100vh'`) sections over dense stacking. Let a heading and a short paragraph sit in the middle of the viewport.
2. **One accent.** Yellow (shade 4) is the only call-to-action color. Everything else is black, white, or grey.
3. **Hairlines, not boxes.** Separate content with black `Divider`s. Avoid cards, borders, and drop shadows except on chart tooltips.
4. **Medium, not bold.** Headings use weight `500`. Display type is large and tight, not heavy.
5. **Round controls, square menus.** Buttons, inputs, and filled icons are fully rounded (`xl`). Menus and overlay icons on photography are square (`radius={0}`).
6. **Photography as structure.** Project images are full-bleed or large squares, not thumbnails in cards.
7. **Charts without chrome.** No tick lines, no grid, signed stacked bars, black in-bar labels.

---

## Personality

| Trait      | How it shows up                                                    |
| ---------- | ------------------------------------------------------------------ |
| Calm       | `grey.0` / `grey.1` page bands, no header border                   |
| Precise    | Black hairline dividers, numeric values to 2 decimals, Inter Tight |
| Optimistic | Mustard yellow CTAs and project 1 identity                         |
| Monumental | H1 up to `4.5rem`, hero images at `100vh`, logo up to `100px`      |
| Honest     | Plain language, FAQ accordion, no marketing gradients              |

Do not introduce: dark mode, colored backgrounds other than grey/yellow/indigo, heavy shadows, outlined cards, or saturated “dashboard” blues as primary.

---

## Foundations

### Font

**Inter Tight** (`Inter Tight, sans-serif`).

- Body and UI: 400
- Headings: **500** (Compare override; `@lcax/ui` defaults to 700 — do not use 700 here)
- Occasional emphasis: 500 on tooltip titles

Loaded via `@fontsource/inter-tight` in `@lcax/ui` (400/500/600/700). Compare also depends on `@fontsource-variable/inter-tight`.

Headings wrap with `overflow-wrap: break-word`.

### Type scale

Mantine font sizes (from `@lcax/ui`):

| Token | Size                |
| ----- | ------------------- |
| `xs`  | 12px                |
| `sm`  | 14px                |
| `md`  | 16px (body default) |
| `lg`  | 18px                |
| `xl`  | 20px                |

Display headings (Compare `theme.module.css`), on top of Mantine defaults:

| Element                  | ≥ `md` (62em)   | ≥ `xl` (88em)                                      |
| ------------------------ | --------------- | -------------------------------------------------- |
| `h1` / `Title`           | 3rem (48px)     | 4.5rem (72px)                                      |
| `h2` / `Title order={2}` | 2.5rem (40px)   | 3rem (48px)                                        |
| `h3` / `Title order={3}` | Mantine default | used for supporting headlines and InfoBlock values |

One-off display sizes on details/data views:

- Project name on photo overlay: `46px` (`md`) → `64px` (`xl`), white
- Data viewer title: same `46px` / `64px`

**Hierarchy on marketing/landing sections**

1. Eyebrow: default `Text` (`Step 1`, `What is LCAx Compare?`)
2. Section name: `Title order={2}` (`Upload`, `Convert`)
3. Supporting line: `Title order={3}` at 75–100% width
4. Body: `Text`, often capped at `66%` width on `xl+`

**Hierarchy on data screens**

1. Page `Title` (default / display scale)
2. Black `Divider`
3. `Title order={2}` for chart groups
4. `InfoBlock`: small label → divider → `Title order={3}` value + unit in `Text`

### Color

Compare **replaces** yellow and adds `grey` + `indigo`. Black is true `#000` (not `@lcax/ui`’s `#2e2e2e`). White is `#fff`.

**Primary:** `yellow`, shade **4**.

#### Yellow (brand / CTA / project 1)

| Shade | Hex       | Use                                                     |
| ----- | --------- | ------------------------------------------------------- |
| 0     | `#fff9e2` | Lightest stacked-bar segment                            |
| 1     | `#fbf2cf` | Stacked-bar segment                                     |
| 2     | `#f4e3a4` | Stacked-bar segment                                     |
| 3     | `#eed474` | Stacked-bar segment                                     |
| 4     | `#e8c547` | **Primary.** Buttons, upload icons, project-1 total bar |
| 5     | `#e5be32` | Stacked-bar segment                                     |
| 6     | `#e4ba22` | Stacked-bar segment                                     |
| 7     | `#caa312` | Darker yellow                                           |
| 8     | `#b49107` | Darker yellow                                           |
| 9     | `#9b7d00` | Darkest yellow                                          |

#### Grey (canvas / project 2)

| Shade | Hex       | Use                                       |
| ----- | --------- | ----------------------------------------- |
| 0     | `#f5f5f5` | Page background, header                   |
| 1     | `#e7e7e7` | Alternate band (project list, comparison) |
| 2     | `#d9d9d9` | Demo / illustration panels on landing     |
| 3     | `#b2b2b2` | Mid grey                                  |
| 4     | `#9a9a9a` | Mid grey                                  |
| 5     | `#8b8b8b` | Mid grey                                  |
| 6     | `#848484` | Mid grey                                  |
| 7     | `#717171` | Mid grey                                  |
| 8     | `#656565` | Dark grey                                 |
| 9     | `#575757` | Project-2 total bar, darkest grey         |

#### Indigo (project 3)

| Shade | Hex       | Use                         |
| ----- | --------- | --------------------------- |
| 0     | `#f1f2f9` | Lightest indigo bar segment |
| 1     | `#e0e2eb` |                             |
| 2     | `#bdc2d9` |                             |
| 3     | `#97a0c7` |                             |
| 4     | `#7883b8` |                             |
| 5     | `#6470af` |                             |
| 6     | `#5967ac` |                             |
| 7     | `#4a5797` |                             |
| 8     | `#414d87` |                             |
| 9     | `#354278` | Project-3 total bar         |

`@lcax/ui` also ships `blue`, `green`, `red`, and `light`. Compare uses **`red` for errors** (`Text c='red'`, alert icons). Do not use `blue` or `green` as brand colors in this app (green only appears as a chart fallback if a project has no `metaData.color`).

#### Semantic roles

| Role                        | Token                        |
| --------------------------- | ---------------------------- |
| Page canvas                 | `grey.0`                     |
| Recessed band               | `grey.1`                     |
| Illustration well           | `grey.2`                     |
| Header                      | `grey.0`, no border          |
| Text / icons on light       | `black` (`#000`)             |
| Text / icons on photography | `white`                      |
| Primary action              | `yellow.4` fill, `c='black'` |
| Divider                     | `black`                      |
| Danger                      | `red` (Mantine / UI palette) |
| Overlay on missing project  | photo + `Overlay` blur `15`  |

#### Project identity

Projects are assigned colors in order: **`yellow` → `grey` → `indigo`** (max 3).

- Totals chart: yellow uses shade **4**; grey and indigo use shade **9**
- Stacked breakdowns: `${projectColor}.${index % 10}` so a single project’s segments walk its full scale

### Radius

| Token                    | Where                                                      |
| ------------------------ | ---------------------------------------------------------- |
| **`xl` (theme default)** | Buttons, FileInput, filled ActionIcons, most controls      |
| `md`                     | Chart tooltip `Paper`                                      |
| `0`                      | `Menu`, overlay ActionIcons on photography, some nav icons |

Filled circular icon buttons: `ActionIcon variant='filled' radius='xl' color='yellow.4'` with a black Tabler icon.

### Elevation

Almost none.

- Chart tooltips: `Paper` with `withBorder`, `shadow='md'`, `radius='md'`, padding `px='md' py='sm'`
- No cards, no header shadow, no modal language on primary flows

### Breakpoints

Shared with `@lcax/ui` (also in `postcss.config.cjs`):

| Token | em    | px (16px root) |
| ----- | ----- | -------------- |
| `xs`  | 36em  | 576            |
| `sm`  | 48em  | 768            |
| `md`  | 62em  | 992            |
| `lg`  | 75em  | 1200           |
| `xl`  | 88em  | 1408           |
| `xxl` | 110em | 1760           |

`--mantine-breakpoint-xxl` is injected via `cssVariablesResolver`.

Responsive values use `useMatches` from `@lcax/ui` (mobile-first: pick the largest matching breakpoint key, else `base`).

### Container widths

Compare overrides Mantine `Container` sizes:

| Size    | Width  |
| ------- | ------ |
| `xs`    | 540px  |
| `sm`    | 720px  |
| `md`    | 960px  |
| `lg`    | 1140px |
| `xl`    | 1320px |
| `xxl`   | 1560px |
| `fluid` | 100%   |

**Typical content width**

```ts
useMatches({ base: 'md', xl: 'xl', xxl: 'xxl' })
// or, when base is unset (full bleed until md):
useMatches({ md: 'md', xl: 'xl', xxl: 'xxl' })
```

Outer page wrappers are `Container fluid p={0}` with `bg='grey.0'` (or `grey.1` for a band). Inner content is a sized `Container`. Horizontal page margins on small screens: `mx={{ base: 'md', md: 'auto' }}`.

---

## Layout

### App shell

- `AppShell` with a collapsing header (`useHeadroom({ fixedAt: 120 })`)
- Header height: `50` → `lg: 65` → `xxl: 100` (same as logo height)
- Header: `withBorder={false}`, `pl='lg'`, `bg='grey.0'`
- Main offset: `pt={headerHeight}` (no extra padding)
- Logo is the only header content; it links home

### Vertical rhythm

Pages are **stacked full-viewport chapters**, not a continuous article.

| Pattern              | Height                                              |
| -------------------- | --------------------------------------------------- |
| Hero / upload        | `h='100vh'`                                         |
| Landing feature rows | `base: 100vh`, `md: 50vh`, `xxl: 30vh`, `mih={400}` |
| Get-started          | `base: 100vh`, `md: 50vh`, `xl: 30vh`               |
| Sponsors             | `h='50vh'`                                          |
| Project block        | `mih` `base: 100vh`, `md: 65vh`, `xxl: 50vh`        |
| Compare / FAQ        | `mih='100vh'`                                       |
| Details hero image   | `h='100vh'` full bleed                              |

Section padding: `py='xl'` on content containers; `my='xl'` on dividers; `mt='xl'` before FAQ / project bands.

### Grid

- Landing features: `SimpleGrid cols={{ base: 1, md: 2 }}`, full remaining height
  - **Left:** copy stack, `mx={{ base: 'md', md: 'xl' }}`
  - **Right:** `bg='grey.2'`, centered illustration (file input or chart), `mih={300}`
- Project block: `SimpleGrid cols={{ base: 1, md: 2 }}` — square photo + facts
- Details facts: two-column on `md+`, nested `SimpleGrid cols={2}` for compact fields
- Info pairs: `cols={2}`

### Alignment

- Marketing heroes and upload: vertically centered (`Stack h='100%' justify='center'`)
- CTAs: left-aligned in heroes, centered in “Get Started”
- Download / breakdown controls: right-aligned (`Flex justify='flex-end'`, `Group justify='space-between'`)
- Empty / 404: centered title + large back icon

---

## Components

### Buttons

**Primary (Get started, download ZIP)**

```tsx
<Button c='black' size='xl' w='fit-content' rightSection={<IconArrowRight />} my='xl'>
  Get started
</Button>
```

- Fill = primary yellow (default `Button`)
- Label and icon = black
- Size `xl` for marketing; `md` for in-content actions (FAQ download)
- Width hugs content
- Icon on the right: `IconArrowRight` (forward) or `IconArrowDown` (download)

**Text / unstyled actions**

- Underlined `UnstyledButton` for FAQ jumps (`style={{ textDecoration: 'underline' }}`)
- Download label is an `UnstyledButton` in a `Button.Group` next to a chevron menu — looks like a text link, not a filled button

Do not use outline or light variants for primary CTAs.

### Icon buttons

| Context          | Spec                                                                                                      |
| ---------------- | --------------------------------------------------------------------------------------------------------- |
| Upload / convert | `variant='filled' radius='xl' color='yellow.4'` size `md`→`xl`, black icon (`IconArrowUp`, `IconLoader2`) |
| Menus            | `variant='transparent' color='black'` + `IconChevronDown`; parent `Menu radius={0}`                       |
| Photo overlay    | `variant='transparent' color='white' size='xl' radius={0}`, Tabler size `64`                              |
| 404 / missing    | Transparent back arrow, icon size `64`–`120`                                                              |

Icons: **Tabler** (`@tabler/icons-react`). Strokey, consistent, never filled brand marks except `IconPointFilled` in tooltips.

### Inputs

`FileInput`

- Width `100%` in the real upload; `50%` in landing demos
- Size `md` → `xl` from `md` up
- Placeholder: `Upload LCA file(s)`
- Right section is the circular yellow ActionIcon (upload trigger)
- Disabled on `base` (mobile upload is intentionally off)
- Errors: input `error` string plus supporting `Text c='red' size='sm'`

No other form controls are part of the visual system.

### Menus

- `Menu radius={0}` (sharp dropdown)
- Trigger is a transparent chevron, often beside a `Text` label (`Breakdown: Total`)
- Items are plain `Menu.Item` labels — no icons, no extra color

### Accordion (FAQ)

- Default Mantine Accordion, `multiple`
- Control icon: `IconInfoCircle`
- Panel: body `Text`; optional `Title order={3}` “Video” + `Divider` + `AspectRatio` `16/10` player; optional primary download button

### InfoBlock

Canonical metadata atom:

```
Label (Text)
————————————  (Divider, black)
Value (Title order={3})   unit (Text, baseline-aligned)
```

Fallback value: `No {title} Given`. Gap `xs` in the stack.

### Dividers

Always default (black). Used as:

- Chapter rules (`my='xl'` or `py='lg'`) under titles
- Rules inside InfoBlocks
- Rules under FAQ / comparison headings (`mt='sm' mb='xl'`)

### Images

- **Logo:** LCAx SVG, height matches header, `w='auto'`
- **Project stills:** Unsplash architectural photos (apartments, office, house), assigned in rotation with colors
- **List thumbnail:** square, `fit='cover'`, size `250` → `350` → `500` (`md` / `xl` / `xxl`)
- **Details hero:** full viewport, `fit='cover'`, white title and icons over the photo (`Overlay` with `backgroundOpacity={0}`)
- **Sponsors:** logos in a `Group gap='xl'`, height `60` → `100` → `150`
- Missing project: same photo language with blur `15`

Photography is documentary / building-scale. No illustrations, no gradients, no stock-people heroes.

### Charts

Mantine `BarChart` (`@mantine/charts` + Recharts).

**Shared**

- Unit copy: `kg CO₂-eq/m²·year`
- Values: `toFixed(2)`
- `barGap: 20`
- Signed stacks: `stackOffset: 'sign'` (module D can be negative)
- Legend at bottom; comparison totals only show legend in “Total” mode
- Tooltip: bordered `Paper`, title `fw={500}`, rows of `IconPointFilled` + `Text fz='sm'`

**Totals comparison (vertical grouped bars)**

- Height `75vh`
- Y-axis label only; default axes otherwise
- Series colors = project identity (yellow.4 / grey.9 / indigo.9)

**Breakdown / details (horizontal stacked)**

- Height `40vh` (`20vh` on `xxl`)
- `orientation='vertical'`, `tickLine='none'`, `gridAxis='none'`
- No Y axis; X domain `['dataMin', 'dataMax']`
- In-bar labels, `fill: 'black'`
- Series = shades of the project color

Landing demo charts follow the same rules at a fixed `h={300}`, often `withTooltip={false}`.

### Tree (data viewer)

- Title + back icon + `Divider`
- `Tree` with `levelOffset={23}`
- Nodes: chevron rotates 180° when open; keys as `Title order={3}` (humanized camelCase) and values as `Text`

### Navigation

- Logo → `/`
- Project name and image → details
- Transparent arrows for back / print / raw JSON
- In-page scroll: `scrollIntoView({ behavior: 'smooth' })` to `#faq` or `#projectN`
- Header hides on scroll down, returns near top (`fixedAt: 120`)

---

## Page recipes

### Landing

1. Full-viewport grey.0 hero: eyebrow, display title, 66%-width body, primary button
2. Repeating two-column chapters (Upload → Convert → Compare → Analyse), each opening with a black divider
3. Centered “Get Started Now!” + same primary button
4. Sponsor row, vertically centered, no divider band

### Projects

1. Full-viewport centered Upload title + file input + FAQ link
2. `grey.1` band of project chapters (photo | facts | text download)
3. Comparison chart (hidden until ≥ 2 projects)
4. FAQ on `grey.0`

### Project details

1. Full-bleed photo with white UI
2. InfoBlock grid on `grey.0`
3. “Detailed Information” + divider + two horizontal stacked charts

### Empty / 404

Centered display title + oversized transparent back arrow. No illustration, no button fill.

---

## Motion

Very little.

- Header collapse via headroom
- Smooth scroll to FAQ / newly added project
- Accordion expand
- Tree chevron rotate
- Upload ActionIcon `loading` state

No page transitions, no hover elevations, no animated charts beyond Recharts defaults.

---

## Content voice (as it appears in the UI)

- Short, complete sentences. No exclamation marks except “Get Started Now!”
- Steps numbered in plain text (`Step 1`), not badges
- Units always shown next to impact numbers
- Project index: `Project 01` (zero-padded), then the real name as the display title
- Errors are red, specific, and point to FAQ — not toast-heavy

---

## Implementation checklist

When porting this system:

1. Start from `@lcax/ui` `uiTheme` + `resolver`, then apply Compare’s `createTheme` override (yellow / grey / indigo, `primaryColor: 'yellow'`, `primaryShade: 4`, `black: '#000'`, `defaultRadius: 'xl'`, heading weight `500`, container sizes, black `Divider`, heading CSS).
2. Put every page in `Container fluid bg='grey.0' p={0}`.
3. Size inner content with `useMatches({ md: 'md', xl: 'xl', xxl: 'xxl' })`.
4. Separate chapters with black `Divider`s and viewport-height stacks — not cards.
5. Primary actions: yellow fill, black label, `size='xl'`, trailing Tabler icon.
6. Assign sequential project colors `yellow | grey | indigo` and drive charts from those tokens.
7. Keep menus square and controls pill-shaped.

---

## Token dump (copy-paste)

```ts
defaultRadius: 'xl'
primaryColor: 'yellow'
primaryShade: 4
black: '#000'
white: '#fff'
fontFamily: 'Inter Tight, sans-serif'
headings.fontWeight: '500'

yellow: ['#fff9e2','#fbf2cf','#f4e3a4','#eed474','#e8c547','#e5be32','#e4ba22','#caa312','#b49107','#9b7d00']
grey:   ['#f5f5f5','#e7e7e7','#d9d9d9','#b2b2b2','#9a9a9a','#8b8b8b','#848484','#717171','#656565','#575757']
indigo: ['#f1f2f9','#e0e2eb','#bdc2d9','#97a0c7','#7883b8','#6470af','#5967ac','#4a5797','#414d87','#354278']

breakpoints: { xs: '36em', sm: '48em', md: '62em', lg: '75em', xl: '88em', xxl: '110em' }
containers:  { xs: 540, sm: 720, md: 960, lg: 1140, xl: 1320, xxl: 1560 }
fontSizes:   { xs: 12, sm: 14, md: 16, lg: 18, xl: 20 } // px via rem()
```
