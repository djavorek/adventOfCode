package hu.javorekdenes.adventOCode.input;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

public class InputUtils {
    public static List<String> getStringsFromFile(String filename) {
        List<String> lines;
        try {
            lines = Files.readAllLines(Paths.get(filename));
            String[] arr = lines.toArray(new String[0]);
            return new ArrayList<>(Arrays.asList(arr));
        } catch (IOException e) {
            e.printStackTrace();
        }

        return List.of();
    }

    /**
     * Converts list of strings, all containing comma separated numbers, into list of integers.
     * @param inputListWithNumbers List of Strings, all containing only comma separated numbers.
     * @return List of Integers, that contains all the numbers from the input Strings.
     */
    public static List<Integer> integersFromInput(List<String> inputListWithNumbers) {
        List<String> stringNumbers = new ArrayList<>();

        for (var input : inputListWithNumbers) {
            stringNumbers.addAll(List.of(input.split(",")));
        }

        return stringNumbers.stream().map(Integer::valueOf).collect(Collectors.toList());
    }
}
