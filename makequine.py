filename="template.rs"
f=open(filename,"r")
content=f.read()
f.close()
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
    elif c == "\n":
        continue
    else:
        print(c, end="")
