import json
import os
import re
import shutil
import stat
import urllib.request

from git import Repo

Repo.clone_from("https://gitlab.com/Melledy/LunarCore-Protos.git", "LunarCore-Protos")
Repo.clone_from(
    "https://github.com/IceDynamix/reliquary-codegen.git", "reliquary-codegen"
)

os.mkdir("data")

urllib.request.urlretrieve(
    "https://raw.githubusercontent.com/Melledy/LunarCore/development/src/main/java/emu/lunarcore/server/packet/CmdId.java",
    "cmdid.java",
)
# stolen with love from https://github.com/hashblen
dict = {}
with open("cmdid.java", "r") as f:
    while f.readline().strip() != "// Cmd Ids":
        pass
    for line in f.readlines():
        line = line.strip()
        name_match = re.search(r"(?<=public static final int )\w+(?= )", line)
        id_match = re.search(r"(?<== )[0-9]+(?=;)", line)
        if name_match is None or id_match is None:
            continue
        dict[id_match[0]] = name_match[0]
with open("data/packetIds.json", "w") as f:
    json.dump(dict, f)
# stolen with love from https://github.com/hashblen

shutil.copytree("LunarCore-Protos/proto", "data/proto")

os.system("cd reliquary-codegen && cargo run -- ../. ../data")


def remove_readonly(func, path, excinfo):
    os.chmod(path, stat.S_IWRITE)
    func(path)


shutil.rmtree("LunarCore-Protos", onexc=remove_readonly)
shutil.rmtree("reliquary-codegen", onexc=remove_readonly)
shutil.rmtree("data", onexc=remove_readonly)

os.remove("cmdid.java")
