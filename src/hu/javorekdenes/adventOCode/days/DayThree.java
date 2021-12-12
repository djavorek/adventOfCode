package hu.javorekdenes.adventOCode.days;

import java.util.Comparator;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class DayThree extends Day {
    @Override
    public Object solveTask1() {
        List<String> inputList = getInputList();

        String mostRecents = IntStream.range(0, inputList.get(0).length())
                .parallel()
                .mapToObj((i) -> inputList.stream()
                        .map((s) -> s.substring(i, i+1))
                        .collect(Collectors.groupingBy(Function.identity(), Collectors.counting()))
                        .entrySet().stream()
                        .max(Map.Entry.comparingByValue())
                        .map(Map.Entry::getKey))
                .map(Optional::get)
                .collect(Collectors.joining(""));

        String leastRecents = mostRecents.chars()
                .mapToObj(c -> c == '1' ? "0" : "1")
                .collect(Collectors.joining(""));

        int decMostRecent = Integer.parseInt(mostRecents, 2);
        int decLeastRecent = Integer.parseInt(leastRecents, 2);

        return decMostRecent * decLeastRecent;
    }

    @Override
    public Object solveTask2() {
        List<String> inputList = getInputList();

        return null;
    }

    @Override
    String getInputFilePathString() {
        return "input3.txt";
    }
}
