import sys

def main():
    if len(sys.argv) != 2:
        print("Usage: python makequine.py <filename>")
        return
    filename=sys.argv[1]
    try:
        f=open(filename,"r")
    except:
        print("File not found")
        return
    content=f.read()
    f.close()

    content2 = ""
    just_hit_newline = False
    for c in content:
        if c == "\n":
            just_hit_newline = True
            content2 += c
        elif just_hit_newline and c == " ": # skip spaces after newline
            continue
        else:
            just_hit_newline = False
            content2 += c

    content = content2

    s = "["
    for i in range(len(content)):
        c = content[i]
        v = ord(c)
        s += f"{v}"
        if i < len(content)-1:
            s += ","
    s += "];"


    for c in content:
        if c == "@":
            print(s, end="")
        #elif c == "\n":
        #    continue
        else:
            print(c, end="")


if __name__ == '__main__':
    main()

