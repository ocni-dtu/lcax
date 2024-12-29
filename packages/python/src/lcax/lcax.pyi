from enum import Enum

# TODO - Test out this: https://crates.io/crates/pyo3-stub-gen


class ProjectPhase(Enum):
    ...


class Country(Enum):
    ...


class Location:
    def __init__(self, country: Country, city=None, address=None):
        ...


class EPD:
    ...


class Project:

    def __init__(self, id: str, name: str, location: Location, project_phase: ProjectPhase, software_info,
                 life_cycle_stages, impact_categories, assemblies, description=None, comment=None, owner=None,
                 format_version=None, lcia_method=None, classification_system=None, reference_study_period=None,
                 results=None, project_info=None, meta_data=None):
        ...


def convert_lcabyg(data: str, result_data: str | None = None) -> Project:
    """Converts a json formatted ILCD+EPD data string into an EPDx formatted json string"""


def convert_ilcd(data: str) -> EPD:
    """Converts a json formatted ILCD+EPD data string into an EPDx formatted json string"""


def calculate_project(project: Project) -> str:
    """Converts a json formatted ILCD+EPD data string into an EPDx formatted json string"""
