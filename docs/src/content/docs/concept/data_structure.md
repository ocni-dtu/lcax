---
title: Data Structure
description: A guide in my new Starlight docs site.
---

## LCAxProject

The project is the top level format that contains information such as name, description, etc.
It also includes information about what life cycle stages and impact categories should be calculated for the project.
Besides that it includes a list of parts (LCAxAssembly items) that make up the project.
Finally, if the project is already calculated then it includes the top level results.

[//]: # (![LCAx Project]&#40;./assets/lcax_structure.png&#41;)

## LCAxAssembly

LCAxAssembly is a format for assemblies/aggregates/build-ups/parts/components.
It is what makes up a project, and it brings together several EPDs in a single unit.
It carries a series of additional data points to elaborate on the EPDs and make them Assembly-specific,
without changing the EPD.

If the project is already calculated then it includes the environmental impact results.

## EPDx

[//]: # (![EPDx]&#40;./assets/epdx.png&#41;)

EPDs is the lowest level of the hierarchy. EPDs contain all the environmental data that is needed to calculate the LCA.
EPDs come in many data formats e.g. ILCD+EPD and PDF, and it is not our intention to replace those formats but meanly
create an exchange format that is easily human and machine-readable.
Besides being a format EPDx is a library for parsing other formats.

The first beta version of EPDx can be found [here](https://epdx.kongsgaard.eu)



---

Overview of LCAx’s Data Model
Objectives of LCAx
The goal for LCAx is to make an open, machine and human-readable data structure and format for exchanging LCA projects, assemblies, impact data and results.
The objective is to create a data structure that can contain information at a building component level, so that the data structure can be used for detailed analysis of the elements that make up a whole building LCA.

Key Principles and Concepts
The LCAx data structure is a nested hierarchy of objects, each with their own set of key-value pairs. The structure is organized into 4 levels:
• Project
• Assembly
• Product
• Impact Data
In the following sections each level will be explained.
The data structure is described in detail with a JSON Schema, which ensures compatibility with modern web standards and ensures field correctness and type validation. The JSON schema can be seen in appendix XX

The 4 levels of LCAx’s data structure

Development of LCAx
The LCAx project was initiated by Christian Kongsgaard and Andreas Sørensen in the spring of 2023 and has since the beginning been developed as an open source project (Apache 2.0) by Christian Kongsgaard ApS.
Besides the data structure, LCAx consists of converters, a validation engine and a calculation engine. These parts of the project are not described here.
As part of the GBDI project it is the intention to contribute to the continued development of LCAx and adapt it to fit the needs of GBDI.

Comparison with Other LCA Data Models
The existing landscape of whole building LCA data models can be divided into two categories:
• Data models for LCA calculation software
• Data models for LCA reporting
All LCA calculation software have an underlying data model. The models are however proprietary, as most the LCA software is, or too tightly coupled to the software. Making it infeasible for a project like GBDI to rely on. These models have, however, a level of detail that makes it possible to analyze information at a building component level, which is required by the GBDI.
Data models used for LCA reporting are typically categorized by aggregating building components into larger groups to enable comparisons across geography and methodologies. These models tend to be open and accessible to allow reporters to hand in data.
The data aggregation comes at the cost of not being able to look into building components and material quantities, making them infeasible for GBDI.
Components of LCAx
As previously mentioned, the data structure of LCAx can be thought of as four different levels: Project, Assembly, Product and Impact Data.
The project level is the outermost level, which contains a collection of assemblies that each have one or more products inside. Each product contains impact data.
This hierarchical system makes the data structure flexible, doesn’t limit the number of elements contained with and mimics the conceptual structure of a building. A building (project) has building components (assembly), which consists of layers (product), each layer being a material (impact data).

Project Level

The project level
The project level contains general information about the project, such as location and owner.
At the project level it is possible to declare information about the nature of the project. For example what classification system, LCIA standard, life cycle stages or what impact categories are considered in the project.
In that way it is possible to get an overview of what is and should be inside the project. It doesn’t mean that declaring for example a classification system prevents other systems from being attached to an assembly, but it declares what is the main system, which must be present at all assemblies.
The same can be said for life cycle stages and impact categories. EPDs usually contain multiple values, but some projects might only care about GWP and A1-A3.
At the project level it is also possible to assign descriptive information about the project in the “Project Info” field. This data can be used to classify and categorize the project for example in connection with data analyses or aggregation.
The fields in “Project Info” are aligned with those of CarbEnMats, to provide an as-complete-as-possible project categorization.
The project level also has a field for LCA results, which are an aggregation of the results at the assembly level.
A complete set of fields for the project level can be seen in appendix XX.

Assembly Level

The assembly level
The Assembly level is made up of a collection of assemblies, which represents a project’s  construction components or built-ups. It holds fields and information related to this level.
Assembly quantities are absolute for the whole project, meaning if an assembly has a quantity of 200m2 then there is 200m2 of this assembly in the whole project.
Each Assembly can have multiple classification systems associated with it, but one has to correspond to the project declared classification.
Assemblies can also contain LCA results, which are the aggregation the LCA results from the contained products.
A complete set of fields for the assembly level can be seen in appendix XX.

Product Level

The product level
A Product is an element of an Assembly. There can be more than one product associated with an assembly.
A Product has a reference service life and an impact data. Transport information can also be associated with the product, which also can hold a piece of impact data.
The quantity of the product is relative to the assembly it belongs to and not absolute to the whole project.
Products can have LCA results, which are aggregations of the impact data values and the product quantity.
A complete set of fields for the product level can be seen in appendix XX
Quantities are relative to the Assembly

Impact Data Level
Impact Data is as the name implies the container for (environmental) impact data. The data can be of different origins. Both EPDs, generic data and tech flows are accepted.
A complete set of fields for the impact data  level can be seen in appendix XX

The impact data level
Component Classification
Component Classification is not tightly coupled to LCAx. LCAx accepts multiple classification schemes in parallel at the assembly level, because it recognizes that it might not be possible to align classification schemes across stakeholders.
However, to be able to effectively categories, filter, sort and organize data the GBDI have adopted the classification schema from Low Carbon Building Initiative (LCBI) - see appendix XX
Advantages and Limitations
The hierarchical structure of the LCAx data model provides flexibility in organizing and representing data at various levels, from project-wide information down to individual product components. This flexibility allows for detailed analysis and customization to suit specific project requirements and stakeholder needs.
The ability to define and store data at a granular level, such as building components and material quantities, enables detailed and precise life cycle assessments. This granularity facilitates accurate environmental impact calculations and supports comprehensive decision-making regarding product design, material selection, and resource optimization.
The standardized data format and schema of LCAx ensure transparency and consistency in data representation and interpretation. Clear documentation of methodologies, assumptions, and data sources enhances the credibility and trustworthiness of LCA results, fostering confidence among users and stakeholders.

While the hierarchical structure gives LCAx a great deal of flexibility it might also pose a challenge for users, particularly those with limited technical expertise or familiarity with data modeling concepts. Adequate training and support may be necessary to fully utilize the capabilities of LCAx and interpret its results accurately.
The reliability and accuracy of LCA results depend on the quality and availability of input data, including life cycle inventory data, impact assessment methodologies, and characterization factors. Data gaps, inconsistencies, or inaccuracies is not something LCAx addresses directly, but will require additional analysis and processing.
Integration with existing LCA software tools or databases may pose compatibility issues due to differences in data formats, standards, or conventions. Efforts to ensure interoperability and data exchange between LCAx and other systems will require resources in the GBDI project.
Conclusion
The LCAx data model offers a flexible, granular, and interoperable framework for analyzing the environmental impacts of products and processes. By providing a standardized and transparent approach to data representation, LCAx promotes consistency, comparability, and reliability in LCA studies, empowering stakeholders to make informed decisions towards sustainability.
Throughout this white paper, we have explored the key principles, components, and advantages of the LCAx data model, as well as its development, implementation, and potential applications. From its inception as an open-source initiative to its ongoing evolution in response to emerging challenges and opportunities, LCAx exemplifies a commitment to innovation, collaboration, and continuous improvement in the pursuit of environmental stewardship.
These properties make LCAx ideal as the underlying data model for LCA projects in GBDI. 