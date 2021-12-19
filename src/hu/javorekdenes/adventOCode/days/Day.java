package hu.javorekdenes.adventOCode.days;

import hu.javorekdenes.adventOCode.Utils;

import java.util.Collections;
import java.util.List;

public abstract class Day {
    public void solve() {
        List<String> inputList = Collections.unmodifiableList(getInputList());
        String currentClassName = this.getClass().toString();
        String taskName = currentClassName.substring(currentClassName.lastIndexOf('.') + 1);

        String result = "******** Results of " + taskName + " ********\n" +
                "* Part One: " + solvePart1(inputList) + "\n" +
                "* Part Two: " + solvePart2(inputList) + "\n" +
                "**********************************\n";

        System.out.print(result);
    }

    abstract Object solvePart1(List<String> inputList);
    abstract Object solvePart2(List<String> inputList);
    abstract String getInputFileName();

    private List<String> getInputList() {
        return Utils.getStringsFromFile(getInputFileName());
    }
}
