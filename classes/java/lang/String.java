package java.lang;

public class String {

    private char[] chars;

    private String() {

    }

    String(char[] chars) {
        this.chars = chars;
    }

    public int length() {
        return chars.length;
    }

    char[] toCharArray() {
        return chars;
    }
}