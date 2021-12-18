package hu.javorekdenes.adventOCode.days;

import java.util.List;
import java.util.stream.Collectors;

public class Day1 extends Day {

    @Override
    String getInputFilePathString() {
        return "input1.txt";
    }

    @Override
    public Integer solveTask1() {
        List<Integer> inputList = getInputList().stream().mapToInt(Integer::valueOf).boxed().collect(Collectors.toList());

        int result = 0;
        int last = Integer.MAX_VALUE;

        for (Integer current : inputList) {
            if (current > last) result++;
            last = current;
        }

        return result;
    }

    @Override
    public Integer solveTask2() {
        List<Integer> inputList = getInputList().stream().mapToInt(Integer::valueOf).boxed().collect(Collectors.toList());

        int result = 0;
        int last = Integer.MAX_VALUE;

        for (int i = 0; i < inputList.size() - 2; i++) {
            int sum = inputList.get(i) + inputList.get(i+1) + inputList.get(i+2);
            if (sum > last) result++;
            last = sum;
        }
        return result;
    }
}
