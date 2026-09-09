import { describe, expect, it } from "vitest";
import { calculateProject, getEnergyAssemblies, Project } from "../../src/lcax";

describe("getEnergyAssemblies", () => {
    it("should return BR18 energy assemblies", () => {
        const assemblies = getEnergyAssemblies("BR18");
        expect(assemblies).toHaveLength(3);

        const expectedCarriers = [
            {
                id: "br18-electricity",
                name: "BR18 Grid Electricity",
                code: "El",
                className: "Operational Electricity",
                productName: "Grid Electricity",
            },
            {
                id: "br18-district-heating",
                name: "BR18 District Heating",
                code: "Fjernvarme",
                className: "Operational Heating",
                productName: "District Heating",
            },
            {
                id: "br18-natural-gas",
                name: "BR18 Natural Gas",
                code: "Ledningsgas",
                className: "Operational Gas",
                productName: "Natural Gas",
            },
        ];

        expectedCarriers.forEach((expected, i) => {
            const assembly = assemblies[i];
            expect(assembly.id).toBe(expected.id);
            expect(assembly.name).toBe(expected.name);
            expect(assembly.unit).toBe("kwh");
            expect(assembly.quantity).toBe(1.0);

            expect(assembly.classification).toHaveLength(1);
            expect(assembly.classification![0].system).toBe("BR18");
            expect(assembly.classification![0].code).toBe(expected.code);
            expect(assembly.classification![0].name).toBe(expected.className);

            expect(assembly.products).toHaveLength(1);
            const product = assembly.products[0] as any;
            expect(product.name).toBe(expected.productName);
            expect(product.unit).toBe("kwh");
            expect(product.quantity).toBe(1.0);
            expect(product.referenceServiceLife).toBe(50);
            expect(product.description).toBe("Impact data should be linearly interpolated");

            expect(product.metaData).toBeDefined();
            expect(product.metaData.isAnnual).toBe(false);

            expect(product.impactData).toHaveLength(5);
            const expectedYears = [2023, 2025, 2030, 2035, 2040];
            expectedYears.forEach((year, j) => {
                const milestone = product.impactData[j];
                expect(milestone.metaData?.year).toBe(year);
            });
        });
    });

    it("should handle standard case-insensitively", () => {
        expect(getEnergyAssemblies("br18")).toHaveLength(3);
        expect(getEnergyAssemblies("Br18")).toHaveLength(3);
    });

    it("should throw error for unsupported standard", () => {
        expect(() => getEnergyAssemblies("unknown")).toThrowError(/Unsupported standard: unknown/);
        expect(() => getEnergyAssemblies("LEED")).toThrowError(/Unsupported standard: LEED/);
    });

    it("should calculate emissions correctly in a project without double scaling", () => {
        const assemblies = getEnergyAssemblies("BR18");
        const electricityAssembly = assemblies[0];
        (electricityAssembly.products[0] as any).quantity = 1000.0;

        const project: Project = {
            id: "project-energy-test",
            name: "Energy Test Project",
            description: null,
            comment: null,
            location: {
                country: "dnk" as any,
                city: null,
                address: null,
            },
            owner: null,
            formatVersion: "2.0.0",
            lciaMethod: null,
            classificationSystems: null,
            referenceStudyPeriod: 50,
            lifeCycleModules: ["b6"],
            impactCategories: ["gwp"],
            assemblies: [{ type: "assembly", ...electricityAssembly } as any],
            results: null,
            projectInfo: null,
            projectPhase: "post_completion",
            softwareInfo: {
                lcaSoftware: "lcax",
            },
            metaData: null,
        };

        const result = calculateProject(project);
        expect(result.results).toBeDefined();
        const gwpB6 = result.results!.gwp.b6;
        expect(gwpB6).toBeCloseTo(50.643, 3);
    });
});
