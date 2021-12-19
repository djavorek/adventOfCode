package hu.javorekdenes.adventOCode.days;

import java.util.*;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Day3 extends Day {
    @Override
    public Object solvePart1(final List<String> inputList) {
        String mostRecent = IntStream.range(0, inputList.get(0).length())
                .parallel()
                .mapToObj((i) -> inputList.stream()
                        .map((s) -> s.substring(i, i+1))
                        .collect(Collectors.groupingBy(Function.identity(), Collectors.counting()))
                        .entrySet().stream()
                        .max(Map.Entry.comparingByValue())
                        .map(Map.Entry::getKey))
                .map(Optional::get)
                .collect(Collectors.joining(""));

        String leastRecent = mostRecent.chars()
                .mapToObj(c -> c == '1' ? "0" : "1")
                .collect(Collectors.joining(""));

        int decMostRecent = Integer.parseInt(mostRecent, 2);
        int decLeastRecent = Integer.parseInt(leastRecent, 2);

        return decMostRecent * decLeastRecent;
    }

    @Override
    public Object solvePart2(final List<String> inputList) {
        List<String> o2gen_max = new ArrayList<>(inputList);
        List<String> co2scrub_min = new ArrayList<>(inputList);

        StringBuilder maxAlreadyFixed = new StringBuilder();
        StringBuilder minAlreadyFixed = new StringBuilder();

        // TODO: Holy Christmas spirit help me out, before someone sees it.
        for (int i = 0; i < inputList.get(0).length(); i++) {
            Map<Character, Integer> characterCountO2 = new HashMap<>();
            Map<Character, Integer> characterCountCO2 = new HashMap<>();
            for (String input : o2gen_max) {
                Character charAtI = input.substring(i, i+1).toCharArray()[0];
                characterCountO2.putIfAbsent(charAtI, 0);
                characterCountO2.put(charAtI, characterCountO2.get(charAtI)+1);
            }
            for (String input : co2scrub_min) {
                Character charAtI = input.substring(i, i+1).toCharArray()[0];
                characterCountCO2.putIfAbsent(charAtI, 0);
                characterCountCO2.put(charAtI, characterCountCO2.get(charAtI)+1);
            }
            int O2charCount0 = characterCountO2.get('0') != null ? characterCountO2.get('0') : 0;
            int CO2charCount0 = characterCountCO2.get('0') != null ? characterCountCO2.get('0') : 0;
            int O2charCount1 = characterCountO2.get('1') != null ? characterCountO2.get('1') : 0;
            int CO2charCount1 = characterCountCO2.get('1') != null ? characterCountCO2.get('1'): 0;
            char mostCommonAtI;
            char leastCommonAtI;

            if (O2charCount0 == O2charCount1) {
                mostCommonAtI = '1';
            } else {
                mostCommonAtI = O2charCount1 > O2charCount0 ? '1' : '0';
            }

            if (CO2charCount0 == CO2charCount1) {
                leastCommonAtI = '0';
            } else {
                leastCommonAtI = CO2charCount1 > CO2charCount0 ? '0' : '1';
            }

            maxAlreadyFixed.append(mostCommonAtI);
            minAlreadyFixed.append(leastCommonAtI);

            if (o2gen_max.size() != 1) {
                o2gen_max = o2gen_max.stream()
                        .filter((input) -> input.startsWith(maxAlreadyFixed.toString()))
                        .collect(Collectors.toList());
            }

            if (co2scrub_min.size() != 1) {
                co2scrub_min = co2scrub_min.stream()
                        .filter((input) -> input.startsWith(minAlreadyFixed.toString()))
                        .collect(Collectors.toList());
            }

            if (o2gen_max.size() == 1 && co2scrub_min.size() == 1) break;
        }


        int decimal_o2gen = Integer.valueOf(o2gen_max.get(0), 2);
        int decimal_co2scrub = Integer.valueOf(co2scrub_min.get(0), 2);

        return decimal_o2gen * decimal_co2scrub;
    }

    @Override
    String getInputFileName() {
        return "input3.txt";
    }
}
