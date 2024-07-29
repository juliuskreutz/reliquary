import json
import os
import shutil
import stat

from git import Repo

Repo.clone_from(
    "https://gitlab.com/YuukiPS/GC-Proto.git",
    "GC-Proto",
    branch="4.7-gc-fix",
)
Repo.clone_from(
    "https://github.com/IceDynamix/reliquary-codegen.git", "reliquary-codegen"
)

os.mkdir("data")


dict = {}
with open("GC-Proto/cmdid.json") as f:
    ids = json.load(f)

    for id in ids:
        dict[id["id"]] = id["name"]
with open("data/packetIds.json", "w") as f:
    json.dump(dict, f)
# stolen with love from https://github.com/hashblen

shutil.copytree("GC-Proto/proto", "data/proto")

os.system("cd reliquary-codegen && cargo run -- ../. ../data")


def remove_readonly(func, path, excinfo):
    os.chmod(path, stat.S_IWRITE)
    func(path)


shutil.rmtree("GC-Proto", onexc=remove_readonly)
shutil.rmtree("reliquary-codegen", onexc=remove_readonly)
shutil.rmtree("data", onexc=remove_readonly)
