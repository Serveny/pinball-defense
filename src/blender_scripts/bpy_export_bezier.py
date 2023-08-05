import bpy
import os


currPath = os.path.splitext(bpy.data.filepath)[0] + ".curves.rs"
file = open(currPath, "w")
file.write("use bevy::prelude::Vec3;\n\n")


def de_casteljau(control_points):
    if len(control_points) == 1:
        return control_points[0]

    new_points = []
    for i in range(len(control_points) - 1):
        x = 0.5 * control_points[i][0] + 0.5 * control_points[i + 1][0]
        y = 0.5 * control_points[i][1] + 0.5 * control_points[i + 1][1]
        z = 0.5 * control_points[i][2] + 0.5 * control_points[i + 1][2]
        new_points.append((x, y, z))

    return de_casteljau(new_points)


def bezier_to_four_points(control_points):
    if len(control_points) < 4:
        raise ValueError("A Bezier curve requires at least 4 control points.")

    p0 = control_points[0]
    p3 = control_points[-1]

    p1 = de_casteljau(control_points)
    p2 = de_casteljau(control_points[::-1])

    return [p0, p1, p2, p3]


def write_bezier_points(matrix, points):
    file.write(f"pub(super) const ROAD_PATH: [[Vec3; 3]; {len(points)}] = [\n")

    for bezier_point in points.values():
        handle_left = matrix @ bezier_point.handle_left
        co = matrix @ bezier_point.co
        handle_right = matrix @ bezier_point.handle_right

        for vec in (handle_left, co, handle_right):
            file.write("[Vec3::new(%.3f, %.3f, %.3f),  " % (vec.x, vec.y, vec.z))

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
