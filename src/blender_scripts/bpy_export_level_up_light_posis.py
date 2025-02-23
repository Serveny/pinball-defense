import os
from pathlib import Path
import bpy
from bpy.types import MeshPolygon, MeshVertex, MeshEdge
from mathutils import Vector, Quaternion

world_name = "world_1"
dir_name = Path(__file__).parent.parent
file_name = os.path.join(dir_name, f"../../src/generated/{world_name}/light_posis.rs")

print("\n-------------------------------------\n")

mesh = bpy.context.active_object
if mesh.type != "MESH":
    raise Exception("Selected object is no mesh")
faces: list[MeshPolygon] = mesh.data.polygons
vertices: list[MeshVertex] = mesh.data.vertices
edges: list[MeshEdge] = mesh.data.edges

with open(file_name, "w") as file:
    file.write(
        f"""// File generated by blender script
use crate::prelude::*;

#[allow(clippy::approx_constant, clippy::excessive_precision)]
pub fn level_up_light_posis() -> [Transform; {len(faces)}] {{ 
    [
"""
    )
    for face in faces:
        pos = face.center
        norm = face.normal
        file.write(
            f"   Transform::from_xyz({pos.x:f}, {pos.y:f}, {pos.z:f}).looking_to(Vec3::new({norm.x:f},{norm.y:f},{norm.z:f}), Vec3::Z),\n"
        )

    file.write("    ]")
    file.write("}")
