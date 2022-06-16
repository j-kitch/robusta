package java.lang;

public class StringIndexOutOfBoundsException extends RuntimeException {

    public StringIndexOutOfBoundsException(int index) {
        super("String index out of range: " + Integer.toString(index));
    }
}
