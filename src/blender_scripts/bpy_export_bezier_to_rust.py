# Export curve to rust/bevy bezier
import bpy

myCurve = bpy.data.curves[0]  # here your curve
spline = myCurve.splines[0]  # maybe you need a loop if more than 1 spline


def vecToStr(vec) -> str:
    pass


print("\n======================")
print("""
// Control points for bezier
let points = [
""")

for point in spline.bezier_points:
    print("    [")
    for vec in (point.co, point.handle_left, point.handle_right):
        print(f"        Vec3::new({vec.x:.02f}, {vec.y:.02f}, {vec.z:.02f}),")
    print("    ],")
print("""];

// Make a CubicCurve
let bezier = Bezier::new(points).to_curve();
""")

