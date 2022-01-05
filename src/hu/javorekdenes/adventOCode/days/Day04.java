package hu.javorekdenes.adventOCode.days;

import hu.javorekdenes.adventOCode.structures.bingo.BingoMatrix;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Iterator;
import java.util.List;

public class Day04 extends Day {
    @Override
    public Object solvePart1(final List<String> inputList) {
        List<Integer> numbersToDraw = new ArrayList<>();

        // First row contains numbers to draw
        String[] stringNumbers = inputList.get(0).split(",");

        Arrays.stream(stringNumbers).forEach((stringNumber) -> {
            int intNumber = Integer.parseInt(stringNumber);
            numbersToDraw.add(intNumber);
        });

        // Upcoming strings contain boards
        List<BingoMatrix> bingoBoards = this.readAllBingoBoards(inputList);

        for (Integer numberToDraw : numbersToDraw) {
            for (BingoMatrix board : bingoBoards) {
                board.markNumber(numberToDraw);

                if (board.isBingo()) {
                    return board.calculateScore() * numberToDraw;
                }
            }
        }

        return 0;
    }

    @Override
    public Object solvePart2(final List<String> inputList) {

        List<Integer> numbersToDraw = new ArrayList<>();

        // First row contains numbers to draw
        String[] stringNumbers = inputList.get(0).split(",");

        Arrays.stream(stringNumbers).forEach((stringNumber) -> {
            int intNumber = Integer.parseInt(stringNumber);
            numbersToDraw.add(intNumber);
        });

        // Upcoming strings contain boards
        List<BingoMatrix> bingoBoards = this.readAllBingoBoards(inputList);
        int wonBoardsCount = 0;
        for (Integer numberToDraw : numbersToDraw) {
            for (BingoMatrix board : bingoBoards) {
                if (board.isBingo()) continue;
                board.markNumber(numberToDraw);

                if (board.isBingo()) {
                    wonBoardsCount++;
                    if (wonBoardsCount == bingoBoards.size()) {
                        return board.calculateScore() * numberToDraw;
                    }
                }
            }
        }

        return 0;
    }

    private List<BingoMatrix> readAllBingoBoards(List<String> rowsOfAllBoard) {
        List<BingoMatrix> bingoBoards = new ArrayList<>();

        Iterator<String> rowIterator = rowsOfAllBoard.iterator();
        while (rowIterator.hasNext()) {
            String row = rowIterator.next();
            if (row.length() == 0 || row.contains(",")) {
                continue; // First or empty row between boards
            }

            List<String> rowsOfBoard = new ArrayList<>();
            rowsOfBoard.add(row);
            for (int i = 0; i < 4; i++) {
                if (rowIterator.hasNext()) {
                    String subsequentRow = rowIterator.next();
                    rowsOfBoard.add(subsequentRow);
                }
            }
            bingoBoards.add(new BingoMatrix(rowsOfBoard));
        }

        return bingoBoards;
    }

    @Override
    String getInputFileName() {
        return "input4.txt";
    }
}
