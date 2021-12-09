package hu.javorekdenes.adventOCode;

import hu.javorekdenes.adventOCode.days.Day;
import hu.javorekdenes.adventOCode.days.DayTwo;

public class App {
    public static void main(String[] args) {
        Day currentDay = new DayTwo();

        System.out.println("Result of task 1 is: " + currentDay.solveTask1());
        System.out.println("Result of task 2 is: " + currentDay.solveTask2());
    }
}
