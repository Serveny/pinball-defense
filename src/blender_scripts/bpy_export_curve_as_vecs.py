import bpy
from mathutils.geometry import interpolate_bezier
from mathutils import Vector
from math import degrees
import os
from pathlib import Path

# Settings
# file_name = os.path.splitext(bpy.data.filepath)[0] + ".curve.rs"

dir_name = Path(__file__).parent.parent
file_name = os.path.join(dir_name, "../../src/road/points.rs")

print("\n-------------------------------------\n")


def get_points(spline, clean=True) -> list[Vector]:
    knots = spline.bezier_points

    if len(knots) < 2:
        return

    # verts per segment
    r = spline.resolution_u + 1

    # segments in spline
    segments = len(knots)

    if not spline.use_cyclic_u:
        segments -= 1

    master_point_list = []
    for i in range(segments):
        inext = (i + 1) % len(knots)

        knot1 = knots[i].co
        handle1 = knots[i].handle_right
        handle2 = knots[inext].handle_left
        knot2 = knots[inext].co

        bezier = knot1, handle1, handle2, knot2, r
        points = interpolate_bezier(*bezier)
        master_point_list.extend(points)

    # some clean up to remove consecutive doubles, this could be smarter...
    if clean:
        old = master_point_list
        good = [v for i, v in enumerate(old[:-1]) if not old[i] == old[i + 1]]
        good.append(old[-1])
        return good

    return master_point_list


def only_relevant_points(points: list[Vector]) -> list[Vector]:
    new_points = [points[0]]
    is_curve = False
    for i in range(len(points) - 2):
        p1, p2, p3 = points[i], points[i + 1], points[i + 2]
        d1, d2 = p2 - p1, p3 - p2
        deg = degrees(d1.angle(d2)) % 180
        if deg > 4:
            if is_curve and d1.length < 0.06 and i % 2 == 1:
                continue
            new_points.append(p2)
            is_curve = True
        else:
            is_curve = False
    new_points.append(points[-1])

    return new_points


spline = bpy.data.curves[0].splines[0]
points = only_relevant_points(get_points(spline))

# Write rust code to file
with open(file_name, "w") as file:
    file.write("use bevy::prelude::Vec3;\n\n")
    file.write(f"pub const ROAD_POINTS: [Vec3; {len(points)}] = [\n")

    for vec in points:
        file.write("    Vec3::new(%.3f, %.3f, %.3f),\n" % (vec.x, vec.z, -vec.y))

    file.write("];\n")

    file.write(f"pub const ROAD_DISTS: [f32; {(len(points) - 1)}] = [\n")

    for i in range(len(points) - 1):
        file.write(f"{    (points[i] - points[i+1]).length:.3f},\n"),

    file.write("];")