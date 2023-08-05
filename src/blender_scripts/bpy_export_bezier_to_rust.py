import bpy


def bezier_to_four_points(bezier_points):
    if len(bezier_points) < 4:
        raise ValueError("A Bezier curve requires at least 4 control points.")

    p0 = bezier_points[0].co
    p3 = bezier_points[-1].co

    p1 = (bezier_points[0].handle_right + bezier_points[1].handle_left) / 2.0
    p2 = (bezier_points[-2].handle_right + bezier_points[-1].handle_left) / 2.0

    return [p0, p1, p2, p3]


# For now, just grab the first curve object
curve = next(filter(lambda obj: obj.type == "CURVE", bpy.data.objects.values()))
bezier_points_list = curve.data.splines[0].bezier_points

# Konvertiere die Bezier-Kurve zu einer Liste mit vier Punkten
converted_points = bezier_to_four_points(bezier_points_list)

print(converted_points)
