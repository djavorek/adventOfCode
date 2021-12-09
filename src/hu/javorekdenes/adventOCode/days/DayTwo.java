package hu.javorekdenes.adventOCode.days;

import java.util.List;

public class DayTwo extends Day {
    @Override
    String getInputFilePathString() {
        return "input2.txt";
    }

    @Override
    public Object solveTask1() {
        List<String> inputList = getInputList();

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
    public Object solveTask2() {
        return null;
    }
}
