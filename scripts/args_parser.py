import argparse


class Args:
    vcd_file: str

    def __init__(self, vcd_file: str):
        self.vcd_file = vcd_file


def parse_args() -> Args:
    parser = argparse.ArgumentParser(description='Parse VCD file')   
    parser.add_argument('--vcd_file', type=str, help='Path to VCD file')

    args = parser.parse_args()
    return Args(vcd_file=args.vcd_file)