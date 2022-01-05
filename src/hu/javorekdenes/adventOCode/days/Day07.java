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

        OptionalDouble optionalAverage = initialHorizontalPositions.stream().mapToInt(Integer::valueOf).average();

        if (optionalAverage.isEmpty()) {
            throw new IllegalArgumentException("Average became empty");
        }
        int roundedAverage = (int)Math.round(optionalAverage.getAsDouble());

        return initialHorizontalPositions.stream()
                .mapToInt((position) -> {
                    int distance = Math.abs(position - roundedAverage);
                    return (distance * (distance + 1)) / 2;
                })
                .sum();

    }

    @Override
    String getInputFileName() {
        return "input7.txt";
    }
}
