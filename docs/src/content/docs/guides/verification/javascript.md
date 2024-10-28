---
title: Example Guide
description: A guide in my new Starlight docs site.
---

# Convert EPD
```typescript
import {convertLCAbyg} from "lcax";
import fs from 'fs'

const epdData = fs.readFileSync(`./ilcd_epd.json`)
const epd = convertIlcd(epdData)

// Output the LCAx formatted EPD to the console
console.log('EPD', epd)
```

# Convert Project

```typescript
import {convertLCAbyg} from "lcax";
import fs from 'fs'

const lcabygData = fs.readFileSync(`./lcabyg_project.json`)
const project = convertLCAbyg(lcabygData)

// Output the LCAx formatted project to the console
console.log('LCAx', project)
```
