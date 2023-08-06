import bpy
import os
from mathutils import Vector

currPath = os.path.splitext(bpy.data.filepath)[0] + ".curves.rs"
file = open(currPath, "w")
file.write("use bevy::prelude::Vec3;\n\n")


def convert_quadratic_to_cubic_bezier(p0, p1, p2):
    # Berechne die neuen Kontrollpunkte für die kubische Bezier-Kurve
    cp0 = p0
    cp1 = Vector(
        ((2 * p0[0] + p1[0]) / 3, (2 * p0[1] + p1[1]) / 3, (2 * p0[2] + p1[2]) / 3)
    )
    cp2 = Vector(
        ((2 * p1[0] + p2[0]) / 3, (2 * p1[1] + p2[1]) / 3, (2 * p1[2] + p2[2]) / 3)
    )
    cp3 = p2

    # Gib die Kontrollpunkte der kubischen Bezier-Kurve zurück
    return [p0, p1, p1, p2]


def write_bezier_points(matrix, points):
    file.write(f"pub const ROAD_PATH: [[Vec3; 4]; {len(points)}] = [\n")

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
