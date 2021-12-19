package hu.javorekdenes.adventOCode.days;

import java.util.List;
import java.util.stream.Collectors;

public class Day1 extends Day {

    @Override
    String getInputFileName() {
        return "input1.txt";
    }

    @Override
    public Integer solvePart1(final List<String> inputList) {
        List<Integer> inputs = inputList.stream().mapToInt(Integer::valueOf).boxed().collect(Collectors.toList());

        int result = 0;
        int last = Integer.MAX_VALUE;

        for (Integer current : inputs) {
            if (current > last) result++;
            last = current;
        }

        return result;
    }

    @Override
    public Integer solvePart2(final List<String> inputList) {
        List<Integer> inputs = inputList.stream().mapToInt(Integer::valueOf).boxed().collect(Collectors.toList());

        int result = 0;
        int last = Integer.MAX_VALUE;

        for (int i = 0; i < inputs.size() - 2; i++) {
            int sum = inputs.get(i) + inputs.get(i+1) + inputs.get(i+2);
            if (sum > last) result++;
            last = sum;
        }
        return result;
    }
}
