from pydantic import BaseModel
from epdx.pydantic import LifeCycleStage, ImpactCategory, Unit

from assembly import Assembly
from results import Results


class AssemblyPart(BaseModel):
    id: str
    name: str
    path: str
    data: Assembly


class Project(BaseModel):
    id: str
    name: str
    description: str
    comment: str
    location: str
    declared_unit: Unit
    quantity: float
    format_version: str
    meta_data: dict
    classification: str
    life_cycle_stages: list[LifeCycleStage]
    impact_categories: list[ImpactCategory]
    parts: list[AssemblyPart]
    results: Results
