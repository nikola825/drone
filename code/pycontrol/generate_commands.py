import sys

file_data = None

with open(sys.argv[1], "r") as commands_file:
    file_data = commands_file.read()

commands = []
for line in file_data.split("\n"):
    line = line.strip()
    if "COMMAND_FUNCTION_DEF" in line:
        line = line[line.find("(")+1:]
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

with open(sys.argv[2], "w") as output_code:
    cmd_index = 0
    print("from commands_base import send_command", file=output_code)
    for command in commands:
        name = command[0]
        args = command[1:]
        assert (len(args) % 2 == 0)
        argcount = len(args)//2
        argnames = []
        format_string = "<"
        for arg_index in range(argcount):
            argtype = args[2*arg_index]
            argname = args[2*arg_index+1]
            assert (argtype in type_to_struct_map)
            format_string += type_to_struct_map[argtype]
            argnames.append(argname)
        argnames = ", ".join(argnames)
        print("", file=output_code)
        print(f"def {name}(sock, {argnames}):", file=output_code)
        print(
            f"    send_command(sock, {cmd_index}, '{format_string}', {argnames})", file=output_code)
        cmd_index += 1
