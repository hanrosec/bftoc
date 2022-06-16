import sys

bf_file = open(sys.argv[1], "r").read()
c_output_file = open(sys.argv[2], "w")

ALLOWED_CHARS = ".,+-<>[]"
def del_unallowed_chars(string):
    for char in string:
        if char not in ALLOWED_CHARS:
            string = string.replace(char, "")
    return string

def bf_to_c(bf_file):
    bf_file = del_unallowed_chars(bf_file)
    c_output = ""
    is_in_loop = False
    for char in bf_file:
        c_output += "\t"
        if char == '>':
            c_output += "++p;\n"            
        elif char == '<':
            c_output += "--p;\n"
        elif char == '+':
            c_output += "++(*p);\n"
        elif char == '-':
            c_output += "--(*p);\n"
        elif char == '.':
            c_output += "putchar(*p);\n"
        elif char == ',':
            c_output += "*p = getchar();\n"
        elif char == '[':
            c_output += "while(*p) {\n"
            is_in_loop = True
        elif char == ']':
            c_output += "}\n"
            is_in_loop = False
        if is_in_loop:
            c_output += "\t"
    return c_output

c_compiled_program = bf_to_c(bf_file)

c_program_template = f"""\
#include <stdio.h>
int main() \x7b
    char memory[30000] = \x7b0\x7d;\n\
    char *p = memory;\n\
{c_compiled_program}
    return 0;
\x7d;"""

print(c_program_template, file=c_output_file)