package hu.javorekdenes.adventOCode;

import hu.javorekdenes.adventOCode.days.Day;
import hu.javorekdenes.adventOCode.days.Day07;

public class App {
    public static void main(String[] args) {
        Day currentDay = new Day07();
        currentDay.solve();
        // currentDay.benchmark();
    }
}
