import {describe, expect, expectTypeOf, it} from "vitest";
import { promises as fs } from 'fs';
import { convertIlcd, EPD} from "../../src/lcax";

describe("convertIlcd", () => {
    it("should convert an ILCD file", async () => {
        const ilcdData = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/f63ac879-fa7d-4f91-813e-e816cbdf1927.json`)).toString()
        const epd = convertIlcd(ilcdData)

        expect(epd).toBeTruthy()
        expectTypeOf(epd).toEqualTypeOf<EPD>()
    })

    it("should throw an error converting the ILCD file", async () => {
        const ilcdData = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/f63ac879_test.json`)).toString()

        expect(() => convertIlcd(ilcdData)).toThrowError('processInformation')
    })
})