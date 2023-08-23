import bpy
from pathlib import Path
import os

dir_name = Path(__file__).parent.parent
dir_name = os.path.join(dir_name, "../../assets/models/gltf")

print("\n-------------------------------------\n")

file_path = os.path.join(dir_name, "world.gltf")
bpy.ops.export_scene.gltf(
    filepath=file_path,
    use_active_scene=True,
    export_materials="EXPORT",
    export_format="GLTF_EMBEDDED",
    export_yup=False,
    export_apply=True,
)
