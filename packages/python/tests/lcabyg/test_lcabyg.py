import json

import pytest

import lcax
from lcax import EPD


def test_lcabyg_parse_project(datafix_dir):
    lcabyg_file = datafix_dir / "lcabyg_project.json"
    project = lcax.convert_lcabyg(lcabyg_file.read_text())

    assert isinstance(project[0], lcax.Project)


def test_lcabyg_parse_project_results(datafix_dir):
    lcabyg_file = datafix_dir / "lcabyg_project.json"
    lcabyg_results_file = datafix_dir / "lcabyg_project_results.json"
    project = lcax.convert_lcabyg(lcabyg_file.read_text(), lcabyg_results_file.read_text())

    assert isinstance(project[0], lcax.Project)


def test_parse_empty():
    with pytest.raises(TypeError):
        lcax.convert_lcabyg("{}")


def test_parse_lcabyg_product(datafix_dir):
    lcabyg_file = datafix_dir / "stages.json"
    epd = lcax.convert_lcabyg(lcabyg_file.read_text())
    assert isinstance(epd[0][0], lcax.EPD)


def test_serialize_to_lcabyg_product(datafix_dir):
    lcax_file = datafix_dir / "epd.json"
    epd = EPD.loads(lcax_file.read_text())

    product_string = lcax.to_lcabyg(epds=[epd])

    assert product_string
