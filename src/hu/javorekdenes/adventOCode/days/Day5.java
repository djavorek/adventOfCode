package hu.javorekdenes.adventOCode.days;

import hu.javorekdenes.adventOCode.structures.Point;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class Day5 extends Day {
    @Override
    Object solvePart1(List<String> inputList) {
        Map<Point, Integer> cloudIntensityMap = new HashMap<>();

        for (String input : inputList) {
            String[] endPoints = input.split("->");

            Point startingPoint = new Point(endPoints[0]);
            Point endingPoint = new Point(endPoints[1]);

            // Leave this branch for the next part, I guess :)
            // TODO: Check Why it alters solution? Two part could be merged without this
            if (!(startingPoint.getX().equals(endingPoint.getX()) || startingPoint.getY().equals(endingPoint.getY()))) {
                continue;
            }

            Set<Point> pointsForInput = Point.getPointsBetween(startingPoint, endingPoint, false);

            for (Point point : pointsForInput) {
                cloudIntensityMap.computeIfPresent(point, (key, oldValue) -> oldValue + 1);
                cloudIntensityMap.putIfAbsent(point, 1);
            }
        }

        return cloudIntensityMap.entrySet().stream()
                .filter((entry) -> entry.getValue() > 1)
                .count();
    }

    @Override
    Object solvePart2(List<String> inputList) {
        Map<Point, Integer> cloudIntensityMap = new HashMap<>();

        for (String input : inputList) {
            String[] endPoints = input.split("->");

            Point startingPoint = new Point(endPoints[0]);
            Point endingPoint = new Point(endPoints[1]);

            Set<Point> pointsForInput = Point.getPointsBetween(startingPoint, endingPoint, true);

            for (Point point : pointsForInput) {
                cloudIntensityMap.computeIfPresent(point, (key, oldValue) -> oldValue + 1);
                cloudIntensityMap.putIfAbsent(point, 1);
            }
        }

        return cloudIntensityMap.entrySet().stream()
                .filter((entry) -> entry.getValue() > 1)
                .count();
    }

    @Override
    String getInputFileName() {
        return "input5.txt";
    }
}
