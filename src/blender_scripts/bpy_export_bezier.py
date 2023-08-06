import bpy
import os
from mathutils import Vector

currPath = os.path.splitext(bpy.data.filepath)[0] + ".curves.rs"
file = open(currPath, "w")
file.write("use bevy::prelude::Vec3;\n\n")


def m(f1: float, f2: float) -> float:
    return (2 * f1 + f2) / 3


def middle(p0: Vector, p1: Vector) -> Vector:
    x = m(p0[0], p1[0])
    y = m(p0[1], p1[1])
    z = m(p0[2], p1[2])
    return Vector(x, y, z)


def convert_quadratic_to_cubic_bezier(
    p0: Vector, p1: Vector, p2: Vector
) -> list[Vector]:
    return [p0, middle(p0, p1), middle(p1, p2), p2]


def write_bezier_points(matrix, points):
    file.write(f"pub(crate)const ROAD_PATH: [[Vec3; 4]; {len(points)}] = [\n")

    for bezier_point in points.values():
        handle_left = matrix @ bezier_point.handle_left
        co = matrix @ bezier_point.co
        handle_right = matrix @ bezier_point.handle_right
        file.write("    [\n")

        for vec in convert_quadratic_to_cubic_bezier(handle_left, co, handle_right):
            file.write("        Vec3::new(%.3f, %.3f, %.3f),\n" % (vec.x, vec.z, vec.y))

        file.write("    ],\n")
    file.write("];\n")


for ob in bpy.data.objects.values():
    if ob.type == "CURVE":
        # file.write('"%s":\n' % ob.name)
        for spline in ob.data.splines:
            points = spline.bezier_points
            if len(points) > 0:
                write_bezier_points(ob.matrix_world, points)
            break
        break

file.close()
