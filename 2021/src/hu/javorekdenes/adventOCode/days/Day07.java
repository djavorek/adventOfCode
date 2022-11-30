package hu.javorekdenes.adventOCode.days;

import hu.javorekdenes.adventOCode.input.InputUtils;

import java.util.List;
import java.util.OptionalDouble;

public class Day07 extends Day {
    @Override
    Object solvePart1(List<String> inputList) {
        List<Integer> initialHorizontalPositions = InputUtils.integersFromInput(inputList);

        initialHorizontalPositions.sort(Integer::compare);

        int middleIndex = initialHorizontalPositions.size() / 2;
        Integer medianValue = initialHorizontalPositions.get(middleIndex);

        return initialHorizontalPositions.stream().mapToInt((position) -> Math.abs(position - medianValue)).sum();
    }

    @Override
    Object solvePart2(List<String> inputList) {
        List<Integer> initialHorizontalPositions = InputUtils.integersFromInput(inputList);

        OptionalDouble optionalMean = initialHorizontalPositions.stream().mapToInt(Integer::valueOf).average();

        if (optionalMean.isEmpty()) {
            throw new IllegalArgumentException("Average cannot be empty, check input");
        }
        // We need to check both ceil and floor, as it is not sure that the rounded average is the most efficient.
        // average - 1/2 < solution < average + 1/2
        final int ceil = (int)Math.ceil(optionalMean.getAsDouble());
        final int floor = ceil - 1;

        int fuelUsageWithFloor = initialHorizontalPositions
                .stream()
                .mapToInt((position) -> {
                    int distance = Math.abs(position - floor);
                    return (distance * (distance + 1)) / 2;
                })
                .sum();

        int fuelUsageWithCeil = initialHorizontalPositions
                .stream()
                .mapToInt((position) -> {
                    int distance = Math.abs(position - ceil);
                    return (distance * (distance + 1)) / 2;
                })
                .sum();


        return Math.min(fuelUsageWithFloor, fuelUsageWithCeil);
    }

    @Override
    String getInputFileName() {
        return "input7.txt";
    }
}
