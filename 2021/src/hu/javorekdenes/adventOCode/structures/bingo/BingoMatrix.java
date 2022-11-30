package hu.javorekdenes.adventOCode.structures.bingo;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Predicate;

public class BingoMatrix {
    private final Map<Integer, Map<Integer, BingoNumber>> rows = new HashMap<>();

    public BingoMatrix(List<String> listOfRows) {
        int rowCounter = 0;
        for (String row : listOfRows) {
            Map<Integer, BingoNumber> columns = new HashMap<>();
            String[] elementsInRow = row.trim().split("\\s+");

            int columnCounter = 0;
            for (String element : elementsInRow) {
                Integer elementValue = Integer.valueOf(element);
                columns.put(columnCounter, new BingoNumber(elementValue, false));
                columnCounter++;
            }

            rows.put(rowCounter, columns);
            rowCounter++;
        }
    }

    public BingoNumber getValueAt(int row, int column) {
        return rows.get(row).get(column);
    }

    public void markNumber(Integer numberToMark) {
        for (Map<Integer, BingoNumber> column : rows.values()) {
            for (BingoNumber number : column.values()) {
                if (number.getNumber().equals(numberToMark)) {
                    number.mark();
                }
            }
        }
    }

    public boolean isBingo() {
        // Yikes, is it possible in O(n) without being too complicated?

        for (Map<Integer, BingoNumber> column : rows.values()) {
            if (column.values().stream().allMatch(BingoNumber::isDrawn)) {
                return true;
            }
        }

        int columnSize = rows.get(0).values().size();
        for (int i = 0; i < columnSize; i++) {
            boolean isFullColumnDrawn = true;
            for (Map<Integer, BingoNumber> column : rows.values()) {
                if (!column.get(i).isDrawn()) {
                    isFullColumnDrawn = false;
                }
            }

            if (isFullColumnDrawn) {
                return true;
            }
        }
        return false;
    }

    public Integer calculateScore() {
        Integer score = 0;
        for (Map<Integer, BingoNumber> column : rows.values()) {
            score += column.values().stream()
                    .filter(Predicate.not(BingoNumber::isDrawn))
                    .mapToInt(BingoNumber::getNumber)
                    .sum();
        }
        return score;
    }
}
