package java.lang;

public class String {

    private char[] chars;

    private String() {

    }

    public int length() {
        return chars.length;
    }

    char[] toCharArray() {
        return chars;
    }
}