from typing import Union

from pydantic import BaseModel, HttpUrl
from epdx.pydantic import Unit, EPD

from results import Results


class Classification(BaseModel):
    name: str
    code: str
    system: str


class ExternalEPD(BaseModel):
    url: HttpUrl
    format: str
    version: str | None


class EPDPart(BaseModel):
    id: str
    name: str
    reference_service_life: float
    epd_source: Union[EPD, ExternalEPD]
    part_quantity: float
    part_unit: Unit


class Assembly(BaseModel):
    id: str
    name: str
    description: str
    comment: str
    quantity: float
    unit: Unit
    category: str
    classification: list[Classification]
    parts: list[EPDPart]
    results: Results
