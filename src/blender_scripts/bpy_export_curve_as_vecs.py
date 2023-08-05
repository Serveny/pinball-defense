import bpy
from mathutils import Vector
import os

# Settings
num_points = 200
file_name = os.path.splitext(bpy.data.filepath)[0] + ".curve.rs"


# Code
def bezier_curve_to_equidistant_points(bezier_points_list, num_points):
    if num_points < 2:
        raise ValueError("The number of points must be at least 2.")

    points = []

    # Berechne die Gesamtlänge der Bezier-Kurve
    total_length = 0.0
    for i in range(len(bezier_points_list) - 1):
        total_length += (bezier_points_list[i].co - bezier_points_list[i + 1].co).length

    segment_length = total_length / (num_points - 1)

    # Initialisiere den aktuellen Punkt und die Bogenlänge
    current_point = bezier_points_list[0].co
    current_length = 0.0

    points.append(current_point)

    # Iteriere über die restlichen Punkte und füge sie hinzu
    for i in range(1, num_points):
        target_length = i * segment_length

        # Suche den nächsten Punkt auf der Kurve, der den äquidistanten Abstand erreicht
        while current_length < target_length:
            current_index = 0
            next_index = 1
            next_point = bezier_points_list[next_index].co
            next_length = current_length + (current_point - next_point).length

            # Beachte die Handles für die Bogenlänge
            while next_length < target_length:
                current_index = next_index
                current_point = next_point
                current_length = next_length

                next_index += 1
                if next_index >= len(bezier_points_list):
                    break

                next_point = bezier_points_list[next_index].co
                next_length = current_length + (current_point - next_point).length

            if next_index < len(bezier_points_list):
                # Interpoliere den Punkt auf der Kurve mit Handles
                remaining_length = target_length - current_length
                t = remaining_length / (next_length - current_length)

                current_handle_right = bezier_points_list[
                    current_index - 1
                ].handle_right
                next_handle_left = bezier_points_list[next_index].handle_left

                # Berechne die Position mit Handles
                p0 = current_point
                p1 = current_point + current_handle_right
                p2 = next_point + next_handle_left
                p3 = next_point

                t2 = t * t
                t3 = t2 * t
                mt = 1 - t
                mt2 = mt * mt
                mt3 = mt2 * mt

                x = mt3 * p0[0] + 3 * mt2 * t * p1[0] + 3 * mt * t2 * p2[0] + t3 * p3[0]
                y = mt3 * p0[1] + 3 * mt2 * t * p1[1] + 3 * mt * t2 * p2[1] + t3 * p3[1]
                z = mt3 * p0[2] + 3 * mt2 * t * p1[2] + 3 * mt * t2 * p2[2] + t3 * p3[2]

                points.append(Vector((x, y, z)))
                current_point = Vector((x, y, z))
                current_length = target_length

    return points


# For now, just grab the first curve object
curve = next(filter(lambda obj: obj.type == "CURVE", bpy.data.objects.values()))
bezier_points_list = curve.data.splines[0].bezier_points


equidistant_points = bezier_curve_to_equidistant_points(bezier_points_list, num_points)

# Write rust code to file
with open(file_name, "w") as file:
    file.write("use bevy::prelude::Vec3;\n\n")
    file.write(f"pub const ROAD_PATH: [Vec3; {num_points}] = [\n")

    for vec in equidistant_points:
        file.write("    Vec3::new(%.3f, %.3f, %.3f),\n" % (vec.x, vec.z, vec.y))

    file.write("];")
