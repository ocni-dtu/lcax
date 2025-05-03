import {describe, expect, expectTypeOf, it} from "vitest";
import { promises as fs } from 'fs';
import { convertIlcd} from "../../src/lcax";

describe("MetaData", () => {
  it("should pull out meta data", async () => {
    const ilcdData = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/f63ac879-fa7d-4f91-813e-e816cbdf1927.json`)).toString()
    const epd = convertIlcd(ilcdData)

    expect(epd.conversions[0].metaData.name).toEqual('gross density')
  })
})