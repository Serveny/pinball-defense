import bpy
from bpy.types import Mesh, Object

objs: list[Object] = bpy.data.objects

print("\n-------------------------------------\n")

for obj in objs:
    if obj.type == "MESH":
        print(obj.name)
        obj.data.name = obj.name

print("\n 8======D \n")
