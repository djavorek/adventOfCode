package hu.javorekdenes.adventOCode.days;

import hu.javorekdenes.adventOCode.Utils;

import java.util.List;
import java.util.stream.Stream;

public abstract class Day {
    public abstract Object solveTask1();
    public abstract Object solveTask2();

    abstract String getInputFilePathString();

    List<String> getInputList() {
        return Utils.getStringsFromFile(getInputFilePathString());
    }

}
