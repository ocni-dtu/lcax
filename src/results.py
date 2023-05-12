from typing import Optional
from epdx.pydantic import ImpactCategory
from pydantic import BaseModel


class Results(BaseModel):
    adpe: Optional[ImpactCategory] = None
    adpf: Optional[ImpactCategory] = None
    ap: Optional[ImpactCategory] = None
    cru: Optional[ImpactCategory] = None
    eee: Optional[ImpactCategory] = None
    eet: Optional[ImpactCategory] = None
    ep: Optional[ImpactCategory] = None
    fw: Optional[ImpactCategory] = None
    gwp: Optional[ImpactCategory] = None
    hwd: Optional[ImpactCategory] = None
    mer: Optional[ImpactCategory] = None
    mrf: Optional[ImpactCategory] = None
    nhwd: Optional[ImpactCategory] = None
    nrsf: Optional[ImpactCategory] = None
    odp: Optional[ImpactCategory] = None
    penre: Optional[ImpactCategory] = None
    penrm: Optional[ImpactCategory] = None
    penrt: Optional[ImpactCategory] = None
    pere: Optional[ImpactCategory] = None
    perm: Optional[ImpactCategory] = None
    pert: Optional[ImpactCategory] = None
    pocp: Optional[ImpactCategory] = None
    rsf: Optional[ImpactCategory] = None
    rwd: Optional[ImpactCategory] = None
    sm: Optional[ImpactCategory] = None
