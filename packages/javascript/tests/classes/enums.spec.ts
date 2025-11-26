import {describe, expect, expectTypeOf, it} from "vitest";
import { promises as fs } from 'fs';
import {
  BuildingModelScope,
  buildingModelScopes, BuildingType, buildingTypes, buildingTypologies, BuildingTypology,
  convertIlcd, countries, Country,
  GeneralEnergyClass,
  generalEnergyClasses, impactCategories, ImpactCategoryKey, LifeCycleModule, lifeCycleModules,
  ProjectPhase,
  projectPhases, RoofType,
  roofTypes, Standard, standards, SubType, subTypes, Unit, units,
} from '../../src/lcax'

describe("Enums", () => {
  it("projectPhases", async () => {
    const values = projectPhases();
    expect(Array.isArray(values)).toBe(true);
    expect(values.length).toBeGreaterThan(0);
    expectTypeOf(values).toMatchTypeOf<ProjectPhase[]>()
  })
  it("roofTypes", async () => {
    const values = roofTypes();
    expect(Array.isArray(values)).toBe(true);
    expect(values.length).toBeGreaterThan(0);
    expectTypeOf(values).toMatchTypeOf<RoofType[]>()
  })
  it("generalEnergyClasses", async () => {
    const values = generalEnergyClasses();
    expect(Array.isArray(values)).toBe(true);
    expect(values.length).toBeGreaterThan(0);
    expectTypeOf(values).toMatchTypeOf<GeneralEnergyClass[]>()
  })
  it("buildingModelScopes", async () => {
    const values = buildingModelScopes();
    expect(Array.isArray(values)).toBe(true);
    expect(values.length).toBeGreaterThan(0);
    expectTypeOf(values).toMatchTypeOf<BuildingModelScope[]>()
  })
  it("buildingTypes", async () => {
    const values = buildingTypes();
    expect(Array.isArray(values)).toBe(true);
    expect(values.length).toBeGreaterThan(0);
    expectTypeOf(values).toMatchTypeOf<BuildingType[]>()
  })
  it("buildingTypologies", async () => {
    const values = buildingTypologies();
    expect(Array.isArray(values)).toBe(true);
    expect(values.length).toBeGreaterThan(0);
    expectTypeOf(values).toMatchTypeOf<BuildingTypology[]>()
  })
  it("lifeCycleModules", async () => {
    const values = lifeCycleModules();
    expect(Array.isArray(values)).toBe(true);
    expect(values.length).toBeGreaterThan(0);
    expectTypeOf(values).toMatchTypeOf<LifeCycleModule[]>()
  })
  it("impactCategories", async () => {
    const values = impactCategories();
    expect(Array.isArray(values)).toBe(true);
    expect(values.length).toBeGreaterThan(0);
    expectTypeOf(values).toMatchTypeOf<ImpactCategoryKey[]>()
  })
  it("units", async () => {
    const values = units();
    expect(Array.isArray(values)).toBe(true);
    expect(values.length).toBeGreaterThan(0);
    expectTypeOf(values).toMatchTypeOf<Unit[]>()
  })
  it("standards", async () => {
    const values = standards();
    expect(Array.isArray(values)).toBe(true);
    expect(values.length).toBeGreaterThan(0);
    expectTypeOf(values).toMatchTypeOf<Standard[]>()
  })
  it("subTypes", async () => {
    const values = subTypes();
    expect(Array.isArray(values)).toBe(true);
    expect(values.length).toBeGreaterThan(0);
    expectTypeOf(values).toMatchTypeOf<SubType[]>()
  })
  it("countries", async () => {
    const values = countries();
    expect(Array.isArray(values)).toBe(true);
    expect(values.length).toBeGreaterThan(0);
    expectTypeOf(values).toMatchTypeOf<Country[]>()
  })
})