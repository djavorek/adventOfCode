package hu.javorekdenes.adventOCode.structures;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Point {
    private final Integer x;
    private final Integer y;

    public Point(final Integer x, final Integer y) {
        this.x = x;
        this.y = y;
    }

    public Point(String commaDelimited) {
        String[] parts = commaDelimited.trim().split(",");
        this.x = Integer.parseInt(parts[0]);
        this.y = Integer.parseInt(parts[1]);
    }

    public Integer getX() {
        return x;
    }

    public Integer getY() {
        return y;
    }

    public static List<Point> getPointsBetweenStraight(Point p1, Point p2) {
        if (!(p1.getX().equals(p2.getX()) || p1.getY().equals(p2.getY()))) {
            throw new IllegalArgumentException("Points are not on the same plane");
        }

        List<Point> result = new LinkedList<>();
        result.add(p1);

        if (p1.getX().equals(p2.getX())) {
            int yDiff = Math.abs(p1.getY() - p2.getY());

        }
    }
}
