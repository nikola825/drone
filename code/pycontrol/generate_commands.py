import sys

file_data = None

commands_input = sys.argv[1]
commands_output = sys.argv[2]

storage_input = sys.argv[3]
storage_output = sys.argv[4]

with open(commands_input, "r") as commands_file:
    commands_input_data = commands_file.read()

commands = []
for line in commands_input_data.split("\n"):
    line = line.strip().replace(" ", "")
    if "COMMAND_FUNCTION_DEF" in line:
        line = line[line.find("(") + 1:]
        line = line[:line.find(")")]
        line = line.strip()
        parts = [x.strip() for x in line.split(",")]
        commands.append(parts)

type_to_struct_map = {
    "uint8_t": "B",
    "uint16_t": "H",
    "uint32_t": "L",
    "float": "f"
}

with open(commands_output, "w") as commands_output_code:
    cmd_index = 0
    print("from commands_base import send_command", file=commands_output_code)
    for command in commands:
        name = command[0]
        args = command[1:]
        assert (len(args) % 2 == 0)
        argcount = len(args) // 2
        argnames = []
        format_string = "<"
        for arg_index in range(argcount):
            argtype = args[2 * arg_index]
            argname = args[2 * arg_index + 1]
            assert (argtype in type_to_struct_map)
            format_string += type_to_struct_map[argtype]
            argnames.append(argname)
        argnames = ", ".join(argnames)
        print("", file=commands_output_code)
        print(f"def {name}(sock, {argnames}):", file=commands_output_code)
        print(
            f"    send_command(sock, {cmd_index}, '{format_string}', {argnames})", file=commands_output_code)
        cmd_index += 1

with open(storage_input, "r") as storage_input_file:
    storage_input_data = storage_input_file.read()

variables = [("storage_start", 0, 4, "uint16_t")]
type_to_size_map = {
    "uint8_t": 1,
    "uint16_t": 2,
    "uint32_t": 4,
    "int8_t": 1,
    "int16_t": 2,
    "int32_t": 4,
    # "float": "f"
}

for line in storage_input_data.split("\n"):
    line = line.strip().replace(" ", "")
    if line.startswith("STOREDVAR("):
        line = line[10:]
        line = line[:line.find(")")]
        line = line.split(",")
        var_type, var_name, previous_name = line
        previous = variables[-1]
        assert (previous[0] == previous_name)
        variables.append((var_name, previous[1] + previous[2], type_to_size_map[var_type], var_type))
        print(variables[-1])

with open(storage_output, "w") as storage_output_code:
    print("from storage_base import write_storage", file=storage_output_code)

    for variable in variables:
        name, address, size, var_type = variable
        print("", file=storage_output_code)
        print(f"def storage_write_{name}(sock, arg_val):", file=storage_output_code)
        print(f"    write_storage(sock, {address}, {size}, arg_val, '{var_type}')", file=storage_output_code)
