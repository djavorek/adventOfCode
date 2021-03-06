package hu.javorekdenes.adventOCode.days;

import java.util.List;

public class Day02 extends Day {
    @Override
    String getInputFileName() {
        return "input2.txt";
    }

    @Override
    public Object solvePart1(final List<String> inputList) {
        int x = 0;
        int depth = 0;

        for (String input : inputList) {
            String[] inputParts = input.split("\\s+");
            String command = inputParts[0];
            int amount = Integer.parseInt(inputParts[1]);

            switch (command) {
                case "forward": {
                    x += amount;
                    break;
                }
                case "down": {
                    depth = depth + amount;
                    break;
                }
                case "up": {
                    depth = Math.max(depth - amount, 0);
                    break;
                }
                default: System.out.println("Invalid input");
            }
        }

        return x * depth;
    }

    @Override
    public Object solvePart2(final List<String> inputList) {
        int x = 0;
        int depth = 0;
        int aim = 0;

        for (String input : inputList) {
            String[] inputParts = input.split("\\s+");
            String command = inputParts[0];
            int amount = Integer.parseInt(inputParts[1]);

            switch (command) {
                case "forward": {
                    x += amount;
                    depth += aim * amount;
                    break;
                }
                case "down": {
                    aim += amount;
                    break;
                }
                case "up": {
                    aim -= amount;
                    break;
                }
                default: System.out.println("Invalid input");
            }
        }

        return x * depth;
    }
}
