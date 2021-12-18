package hu.javorekdenes.adventOCode.structures;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Matrix {
    private final Map<Integer, Map<Integer, Integer>> rows = new HashMap<>();

    public Matrix(List<String> listOfRows) {
        int rowCounter = 0;
        for (String row : listOfRows) {
            Map<Integer, Integer> columns = new HashMap<>();
            String[] elementsInRow = row.split("\\s+");

            int columnCounter = 0;
            for (String element : elementsInRow) {
                Integer elementValue = Integer.valueOf(element);
                columns.put(columnCounter, elementValue);
                columnCounter++;
            }

            rows.put(rowCounter, columns);
            rowCounter++;
        }
    }

    public Integer getValueAt(int row, int column) {
        return rows.get(row).get(column);
    }
}
