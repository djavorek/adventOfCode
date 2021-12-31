package hu.javorekdenes.adventOCode.days;

import hu.javorekdenes.adventOCode.Utils;

import java.util.Collections;
import java.util.List;

public abstract class Day {
    public void solve() {
        List<String> inputList = Collections.unmodifiableList(getInputList());
        String currentClassName = this.getClass().toString();
        String taskName = currentClassName.substring(currentClassName.lastIndexOf('.') + 1);

        Object resultOfP1 = solvePart1(inputList);
        Object resultOfP2 = solvePart2(inputList);

        String result = "******** Results of " + taskName + " ********\n" +
                "* Part One: " + resultOfP1 + "\n" +
                "* Part Two: " + resultOfP2 + "\n" +
                "**********************************\n";

        System.out.print(result);
    }

    public void benchmark() {
        long RUN_MILLIS = 1000;
        int REPEAT = 5;
        int WARMUP = 2;
        int LOOP = 100;

        List<String> inputList = Collections.unmodifiableList(getInputList());
        String currentClassName = this.getClass().toString();
        String taskName = currentClassName.substring(currentClassName.lastIndexOf('.') + 1);

        Utils.bench(taskName + " - Part 1", RUN_MILLIS, LOOP, WARMUP, REPEAT, () -> solvePart1(inputList));

        Utils.bench(taskName + " - Part 2", RUN_MILLIS, LOOP, WARMUP, REPEAT, () -> solvePart2(inputList));
    }

    abstract Object solvePart1(List<String> inputList);
    abstract Object solvePart2(List<String> inputList);
    abstract String getInputFileName();

    private List<String> getInputList() {
        return Utils.getStringsFromFile(getInputFileName());
    }
}
