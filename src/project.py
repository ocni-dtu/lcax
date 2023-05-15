from enum import Enum

from pydantic import BaseModel
from epdx.pydantic import ImpactCategory, Unit

from assembly import Assembly
from results import Results


class LifeCycleStage(Enum):
    a1a3 = "A1A3"
    a4 = "A4"
    a5 = "A5"
    b1 = "B1"
    b2 = "B2"
    b3 = "B3"
    b4 = "B4"
    b5 = "B5"
    b6 = "B6"
    b7 = "B7"
    c1 = "C1"
    c2 = "C2"
    c3 = "C3"
    c4 = "C4"
    d = "D"


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
