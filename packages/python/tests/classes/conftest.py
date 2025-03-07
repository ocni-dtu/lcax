import pytest

@pytest.fixture
def epd_file(datafix_dir):
    yield datafix_dir / "epd.json"


@pytest.fixture
def epd(epd_file):
    from lcax import EPD

    yield EPD.loads(epd_file.read_text())
