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
        try {
            return Files.readAllLines(Paths.get(filename));
        } catch (IOException e) {
            System.err.println("Error during reading the input file: " + filename);
            throw new Error("Cannot continue without input.");
        }
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
