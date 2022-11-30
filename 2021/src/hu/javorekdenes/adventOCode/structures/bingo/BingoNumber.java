package hu.javorekdenes.adventOCode.structures.bingo;

public class BingoNumber {
    private final Integer number;
    private Boolean drawn;

    public BingoNumber(final Integer number, final Boolean drawn) {
        this.number = number;
        this.drawn = drawn;
    }

    public Integer getNumber() {
        return number;
    }

    public Boolean isDrawn() {
        return drawn;
    }

    public void mark() {
        this.drawn = true;
    }
}
