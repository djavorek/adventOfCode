package hu.javorekdenes.adventOCode.structures;

import java.util.*;

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

    @Override
    public String toString() {
        return "Point{" +
                "x=" + x +
                ", y=" + y +
                '}';
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Point point = (Point) o;
        return Objects.equals(getX(), point.getX()) && Objects.equals(getY(), point.getY());
    }

    @Override
    public int hashCode() {
        return Objects.hash(getX(), getY());
    }

    public static Set<Point> getPointsBetween(Point p1, Point p2, boolean addDiagonals) {
        Set<Point> result = new HashSet<>();
        result.add(p1);

        if (p1.getX().equals(p2.getX()) && !p1.getY().equals(p2.getY())) { // X equals
            int increment = p1.getY() > p2.getY() ? 1 : -1;
            int targetY = p1.getY();
            int intermediateY = p2.getY();

            while (intermediateY != targetY) {
                intermediateY += increment;
                result.add(new Point(p1.getX(), intermediateY));
            }
        } else if (!p1.getX().equals(p2.getX()) && p1.getY().equals(p2.getY())) { // Y equals
            int increment = p1.getX() > p2.getX() ? 1 : -1;
            int targetX = p1.getX();
            int intermediateX = p2.getX();

            while (intermediateX != targetX) {
                intermediateX += increment;
                result.add(new Point(intermediateX, p1.getY()));
            }
        } else if (addDiagonals) {
            if (Math.abs(p1.getX() - p2.getX()) == Math.abs(p1.getY() - p2.getY())) {
                throw new IllegalArgumentException("Not complete diagonal");
            }

            int incrementX = p1.getX() > p2.getX() ? 1 : -1;
            int incrementY = p1.getY() > p2.getY() ? 1 : -1;

            int targetX = p1.getX();

            // Checking only X target as these will be met in the same iteration
            // int targetY = p1.getY();

            int intermediateX = p2.getX();
            int intermediateY = p2.getY();

            while (intermediateX != targetX) {
                intermediateX += incrementX;
                intermediateY += incrementY;
                result.add(new Point(intermediateX, intermediateY));
            }
        }

        result.add(p2);
        return result;
    }
}
