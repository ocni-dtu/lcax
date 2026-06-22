import {describe, expect, it} from "vitest";
import { getCountryName } from '../../src/lcax'

describe("getCountryName", () => {
  it.each([
    { input: "AT", expected: "Austria"},
    { input: "AUT", expected: "Austria"},
    { input: "Invalid", expected: "Unknown"},
  ])(
    "getCountryName",
    ({ input, expected }) => {
      expect(getCountryName(input)).toBe(expected)
    }
  )
})
