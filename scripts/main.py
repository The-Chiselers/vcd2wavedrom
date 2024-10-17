import json
import sys
from pyDigitalWaveTools.vcd.parser import VcdParser

# Local files
import args_parser as args_parser

def parse_vcd(vcd_file):
    with open(vcd_file) as vcd_file:
        vcd = VcdParser()
        vcd.parse(vcd_file)
        data = vcd.scope.toJson()
        return data
    
def find_signal(vcd_data, signal):
    if vcd_data["name"] == signal:
        return vcd_data["data"]
    if "children" not in vcd_data:
        return None
    for child in vcd_data["children"]:
        result = find_signal(child, signal)
        if result:
            return result
    return None

def data_to_array(data, config):
    start_time = config["start_time"]
    end_time = config["end_time"]
    data_array = []
    # start_time = 3
    # end_time = 10
    # #####################
    # [(0, '0'), (4, '1'), (6, '0'), (9, "1"), ...]
    # ->
    # ["0", "1", ".", "0", ".", ".", "1", ".", ...]
    # [ 3,   4    5    6    7    8    9    10, ...] 
    # i.e. a dot if the value is the same as the previous one

    current_time = start_time
    current_index = 0
    last_time_value = data[-1][0]
    data_index_start = 0
    while data[current_index][0] < start_time:
        current_index += 1
    current_index -= 1

    # This is the first value
    data_array.append(data[current_index][1])
    while current_time < end_time:
        # only increase the index once the current time is greater than the time in the current index
        if current_time >= data[current_index][0]:
            current_index += 1
            if current_index == len(data):
                break
            data_array.append(data[current_index][1])
        else:
            data_array.append(".")
        current_time += 1
    return data_array
    

def data_to_wavedrom_string(data, config):
    wavedrom_string = ""
    data_array = data_to_array(data, config)
    wavedrom_string = "".join(data_array)
    return wavedrom_string

def to_wavedrom(signals, config):
    start_time = config["start_time"]
    end_time = config["end_time"]
    clocks = config["clocks"]
    wavedrom = {
        "signal": []
    }
    
    for signal_name, data in signals.items():
        rename_str = config["signals"][signal_name]["rename"]
        if not signal_name in clocks:
            wavedrom_str = data_to_wavedrom_string(data, config)
            wavedrom["signal"].append({
                "name": rename_str,
                "wave": wavedrom_str
            })
        else:
            wavedrom_str = "p"
            length = end_time - start_time
            for i in range(length):
                wavedrom_str += "."
            wavedrom["signal"].append({
                "name": rename_str,
                "wave": wavedrom_str,
            })
    return wavedrom




def main():
    args = args_parser.parse_args()
    
    vcd_file = args.vcd_file
    vcd_json = parse_vcd(vcd_file)

    print(json.dumps(vcd_json, indent=4))

if __name__ == "__main__":
    main()