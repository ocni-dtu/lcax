import json
from typing import Type, TypeVar

from .lcax import *
from .pydantic import *

__doc__ = lcax.__doc__
if hasattr(lcax, "__all__"):
    __all__ = lcax.__all__

T = TypeVar("T", str, dict, LCAxProject)


def convert_lcabyg(data: str | dict, result_data: str | dict | None = None, *, as_type: Type[T] = dict) -> T:
    """
    Converts json formatted LCAbyg data into a LCAx project

    The LCAx project can either be returned as a string, a dict or a Pydantic class.
    """

    if isinstance(data, dict):
        data = json.dumps(data)

    if isinstance(result_data, dict):
        result_data = json.dumps(result_data)

    try:
        _project = lcax._convert_lcabyg(data, result_data)
    except Exception as err:
        raise ParsingException(err)

    if as_type == str:
        return _project
    elif as_type == dict:
        return json.loads(_project)
    elif as_type == LCAxProject:
        return LCAxProject(**json.loads(_project))
    else:
        raise NotImplementedError("Currently only 'dict', 'str' and 'lcax.LCAxProject' is implemented as_type.")


class ParsingException(Exception):
    ...
