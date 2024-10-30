# vcd2wavedrom

This is a script to convert a VCD file to a WaveDrom file.

## Usage

```bash
sh run.sh <input.vcd> <input.json>
```

This will create an output file called wave.svg in the out directory.

### JSON Configuration

This uses a json file to pass additional options to the script like which signals to display and how to rename them.

```json
{
    "signals": [
        {
            "name": "clock",
            "rename": "clock"
        },
        {
            "name": "reset",
            "rename": "reset"
        },
        {
            "name": "io_apb_PSEL",
            "rename": "PSEL"
        },
        {
            "name": "io_apb_PENABLE",
            "rename": "PENABLE"
        },
        {
            "name": "io_apb_PWRITE",
            "rename": "PWRITE"
        },
        {
            "name": "io_apb_PADDR",
            "rename": "PADDR"
        },
        {
            "name": "io_apb_PWDATA",
            "rename": "PWDATA"
        },
        {
            "name": "io_apb_PRDATA",
            "rename": "PWRITE"
        },
        {
            "name": "io_apb_PREADY",
            "rename": "PREADY"
        },
        {
            "name": "io_apb_PSLVERR",
            "rename": "PSLVERR"
        },
        {
            "name": "io_in",
            "rename": "in"
        },
        {
            "name": "io_out",
            "rename": "out"
        },
        {
            "name": "io_enable",
            "rename": "enable"
        },
        {
            "name": "io_irq",
            "rename": "irq"
        }
    ],
    "clocks": {
        "clock": "clock"
    }
}
```