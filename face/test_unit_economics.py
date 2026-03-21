import unittest
from unittest.mock import patch
import io
import sys
import os

# Add the directory containing unit_economics.py to the Python path
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from unit_economics import evaluate_channels

class TestUnitEconomics(unittest.TestCase):
    def test_evaluate_channels_healthy(self):
        data = {
            "channels": {
                "seo": {"cac": 10.0, "ltv": 50.0, "customers": 5}
            }
        }
        with patch('sys.stdout', new=io.StringIO()) as fake_stdout:
            evaluate_channels(data)
            output = fake_stdout.getvalue()

        self.assertIn("Channel: seo | LTV: $50.00 | CAC: $10.00 | Ratio: 5.00", output)
        self.assertNotIn("FLAG_CHANNEL_UNVIABLE", output)

    def test_evaluate_channels_unviable(self):
        data = {
            "channels": {
                "ads": {"cac": 50.0, "ltv": 100.0, "customers": 10}
            }
        }
        with patch('sys.stdout', new=io.StringIO()) as fake_stdout:
            evaluate_channels(data)
            output = fake_stdout.getvalue()

        self.assertIn("Channel: ads | LTV: $100.00 | CAC: $50.00 | Ratio: 2.00", output)
        self.assertIn("FLAG_CHANNEL_UNVIABLE => ads is burning capital. Terminate ad spend.", output)

    def test_evaluate_channels_zero_cac(self):
        data = {
            "channels": {
                "organic": {"cac": 0.0, "ltv": 20.0, "customers": 2}
            }
        }
        with patch('sys.stdout', new=io.StringIO()) as fake_stdout:
            evaluate_channels(data)
            output = fake_stdout.getvalue()

        self.assertIn("Channel: organic | LTV: $20.00 | CAC: $0.00 | Ratio: inf", output)
        self.assertNotIn("FLAG_CHANNEL_UNVIABLE", output)

    def test_evaluate_channels_multiple(self):
        data = {
            "channels": {
                "seo": {"cac": 10.0, "ltv": 50.0, "customers": 5},
                "ads": {"cac": 50.0, "ltv": 100.0, "customers": 10},
                "organic": {"cac": 0.0, "ltv": 20.0, "customers": 2}
            }
        }
        with patch('sys.stdout', new=io.StringIO()) as fake_stdout:
            evaluate_channels(data)
            output = fake_stdout.getvalue()

        # Check all channels are in output
        self.assertIn("Channel: seo | LTV: $50.00 | CAC: $10.00 | Ratio: 5.00", output)
        self.assertIn("Channel: ads | LTV: $100.00 | CAC: $50.00 | Ratio: 2.00", output)
        self.assertIn("Channel: organic | LTV: $20.00 | CAC: $0.00 | Ratio: inf", output)

        # Check that only ads got flagged
        self.assertIn("FLAG_CHANNEL_UNVIABLE => ads is burning capital. Terminate ad spend.", output)
        self.assertNotIn("FLAG_CHANNEL_UNVIABLE => seo", output)
        self.assertNotIn("FLAG_CHANNEL_UNVIABLE => organic", output)

if __name__ == '__main__':
    unittest.main()
