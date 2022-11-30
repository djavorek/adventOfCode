package hu.javorekdenes.adventOCode.days;

import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class Day06 extends Day{
    @Override
    Object solvePart1(List<String> inputList) {
        String[] inputs = inputList.get(0).split(",");

        List<Integer> fish = Arrays.stream(inputs).map(Integer::parseInt).collect(Collectors.toList());

        return getFishPopulationAfterDays(fish, 80);
    }

    @Override
    Object solvePart2(List<String> inputList) {
        String[] inputs = inputList.get(0).split(",");

        List<Integer> fish = Arrays.stream(inputs).map(Integer::parseInt).collect(Collectors.toList());

        return getFishPopulationAfterDays(fish, 256);
    }

    private Long getFishPopulationAfterDays(final List<Integer> initialPopulation, int days) {
        long population = 0;
        long[] fishCountByRemainingDays = new long[9];

        for (var fishInstance : initialPopulation) {
            fishCountByRemainingDays[fishInstance]++;
        }

        for (int day = 0; day < days; day++) {
            long doneFishCount = fishCountByRemainingDays[0];

            for (int i = 1; i < fishCountByRemainingDays.length; i++) {
                fishCountByRemainingDays[i - 1] = fishCountByRemainingDays[i];
            }

            fishCountByRemainingDays[6] += doneFishCount;
            fishCountByRemainingDays[8] = doneFishCount;
        }

        for (long fishCount : fishCountByRemainingDays) {
            population += fishCount;
        }

        return population;
    }

    @Override
    String getInputFileName() {
        return "input6.txt";
    }
}
