import bpy
from mathutils.geometry import interpolate_bezier
import os
from pathlib import Path

# Settings
# file_name = os.path.splitext(bpy.data.filepath)[0] + ".curve.rs"

dir_name = Path(__file__).parent.parent
file_name = os.path.join(dir_name, "../../src/road/points.rs")


def get_points(spline, clean=True):
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


spline = bpy.data.curves[0].splines[0]

points = get_points(spline)[::4]

# Write rust code to file
with open(file_name, "w") as file:
    file.write("use bevy::prelude::Vec3;\n\n")
    file.write(f"pub const ROAD_POINTS: [Vec3; {len(points)}] = [\n")

    for vec in points:
        file.write("    Vec3::new(%.3f, %.3f, %.3f),\n" % (vec.x, vec.z, -vec.y))

    file.write("];")
