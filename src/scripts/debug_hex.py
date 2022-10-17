from math import sin, cos, pi


def inspect(points):
    print("     POINTS: ", points)
    print("     SCALED: ", [scale(point) for point in points])
    print("TRANSFORMED: ", [transform_point(point) for point in points])


def scale(point):
    x = point[0] / 180
    y = point[1] / 90
    return x, y


def transform_point(point):
    x, y = scale(point)
    return (
        sin(x * pi * 0.5) * cos(y * pi * 0.5),
        sin(y * pi * 0.5)
    )


if __name__ == "__main__":
    inspect([(145.000, -81.271), (-34.000, -87.365), (-85.000, -76.163), (-117.000, -69.394), (-148.000, -68.930), (-179.000, -73.310)])
    print()
    inspect([(45.000, -31.286), (37.000, -22.886), (23.000, -24.697), (18.000, -34.994), (26.000, -43.443), (41.000, -41.457)])