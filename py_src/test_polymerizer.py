from pathlib import Path
import pytest
from .polymerization import Polymerizer

# pylint:disable=redefined-outer-name


@pytest.fixture
def polymerizer():
    return Polymerizer(Path("data/day14/test.txt"))


def test_steps(polymerizer: Polymerizer):
    assert polymerizer.chain == "NNCB"

    polymerizer.apply_rules()
    assert polymerizer.chain == "NCNBCHB"
    polymerizer.apply_rules()
    assert polymerizer.chain == "NBCCNBBBCBHCB"
    polymerizer.apply_rules()
    assert polymerizer.chain == "NBBBCNCCNBBNBNBBCHBHHBCHB"
    polymerizer.apply_rules()
    assert polymerizer.chain == "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"


def test_answer(polymerizer: Polymerizer):
    for _ in range(0, 10):
        polymerizer.apply_rules()

    assert polymerizer.answer() == 1588
