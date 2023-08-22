import bpy
from bpy.types import Collection
from pathlib import Path
import os
import re

dir_name = Path(__file__).parent.parent
dir_name = os.path.join(dir_name, "../../assets/models/gltf")

print("\n-------------------------------------\n")


def select_all_from_collection(coll: Collection):
    bpy.ops.object.select_all(action="DESELECT")
    for obj in coll.all_objects:
        obj.select_set(True)


def to_snake_case(string: str):
    return re.sub(r"(?<!^)(?=[A-Z])", "_", string).replace(" ", "").lower()


def is_root(coll_to_check: Collection) -> bool:
    for coll in bpy.data.collections:
        if coll_to_check.name in map(lambda c: c.name, coll.children_recursive):
            print(f"collection {coll.name} is no root")
            return False
    return True


for coll in bpy.data.collections:
    if not is_root(coll):
        continue
    print(f"Export collection: {coll.name}")
    select_all_from_collection(coll)
    file_path = os.path.join(dir_name, f"{to_snake_case(coll.name)}.gltf")
    bpy.ops.export_scene.gltf(
        filepath=file_path,
        use_selection=True,
        export_materials="EXPORT",
        export_format="GLTF_EMBEDDED",
        export_yup=False,
    )
