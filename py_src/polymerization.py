from dataclasses import dataclass
from pathlib import Path


@dataclass
class Polymerizer:
    # Chain of polymers. Example: NNCB
    chain: str
    """ Polymerization rules of the form "CH": "B", meaning that rule turns
        any pair CH into CBH """
    rules: dict[str, str]

    def __init__(self, file: Path):
        with open(file, "r", encoding="utf-8") as fp:
            lines = fp.readlines()
            self.chain = lines[0].strip()

            self.rules = {}
            for rule_string in lines[2:]:
                parts = rule_string.split(" -> ")
                self.rules[parts[0]] = parts[1].strip()

    def apply_rules(self):
        pairs = self._split_to_pairs()
        self.chain = self._glue([self._maybe_apply_rule(pair) for pair in pairs])

    # Splits the current chain into pairs of strings
    def _split_to_pairs(self) -> list[str]:
        pairs: list(str) = []

        for index in range(0, len(self.chain) - 1):
            pair = self.chain[index : index + 2]
            pairs.append(pair)

        return pairs

    # Apply relevant rule to pair if it exists.
    def _maybe_apply_rule(self, pair: str) -> str:
        try:
            insertion = self.rules[pair]
            return pair[0] + insertion + pair[1]
        except KeyError:
            return pair

    # Glue together the pairs
    @staticmethod
    def _glue(pairs: list[str]) -> str:
        chain = [pair[:-1] for pair in pairs[:-1]] + pairs[-1:]
        return "".join(chain)

    # Return answer to challenge question: qty of most common - qty of least common element
    def answer(self) -> int:
        frequencies: dict[str, str] = {}

        for char in self.chain:
            try:
                frequency = frequencies[char]
            except KeyError:
                frequency = 0

            frequencies[char] = frequency + 1

        return max(frequencies.values()) - min(frequencies.values())
