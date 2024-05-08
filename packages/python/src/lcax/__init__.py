import json
from pathlib import Path
from typing import Type, TypeVar

from .lcax import *
from .pydantic import *

__doc__ = lcax.__doc__
if hasattr(lcax, "__all__"):
    __all__ = lcax.__all__

Project_Type = TypeVar("Project_Type", str, dict, Project)


def convert_lcabyg(data: str | dict, result_data: str | dict | None = None, *, as_type: Type[Project_Type] = dict) -> Project_Type:
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
    elif as_type == Project:
        return Project(**json.loads(_project))
    else:
        raise NotImplementedError("Currently only 'dict', 'str' and 'lcax.LCAxProject' is implemented as_type.")


EPD_Type = TypeVar("EPD_Type", str, dict, EPD)


def convert_ilcd(data: str | dict, *, as_type: Type[EPD_Type] = dict) -> EPD_Type:
    """
    Converts a json formatted ILCD+EPD data into EPDx

    The EPDx data can either be returned as a string, a dict or a Pydantic class.
    """

    if isinstance(data, dict):
        data = json.dumps(data)

    try:
        _epd = lcax._convert_ilcd(data)
    except Exception as err:
        raise ParsingException(err)

    if as_type == str:
        return _epd
    elif as_type == dict:
        return json.loads(_epd)
    elif as_type == EPD:
        return EPD(**json.loads(_epd))
    else:
        raise NotImplementedError("Currently only 'dict', 'str' and 'lcax.EPD' is implemented as_type.")


def convert_slice(path: str | Path, *, as_type: Type[Project_Type] = dict) -> list[Project_Type]:
    """
    Converts a SLiCE .parquet file into a list of LCAx projects

    The LCAx project can either be returned as a list of strings, dicts or Pydantic classes.
    """

    if isinstance(path, str):
        path = Path(path)

    if not path.is_file():
        raise FileNotFoundError(f"File not found: {path}")

    try:
        projects = lcax._convert_slice(str(path))
    except Exception as err:
        raise ParsingException(err)

    if as_type == str:
        return projects
    elif as_type == dict:
        return [json.loads(project) for project in projects]
    elif as_type == EPD:
        return [Project(**json.loads(project)) for project in projects]
    else:
        raise NotImplementedError("Currently only 'dict', 'str' and 'lcax.EPD' is implemented as_type.")


class ParsingException(Exception):
    ...
