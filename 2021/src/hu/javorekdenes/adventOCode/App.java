package hu.javorekdenes.adventOCode;

import hu.javorekdenes.adventOCode.days.Day;
import hu.javorekdenes.adventOCode.days.Day08;

public class App {
    public static void main(String[] args) {
        Day currentDay = new Day08();
        currentDay.solve();
        // currentDay.benchmark();
    }
}
