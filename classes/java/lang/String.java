package java.lang;

public class String {

    private char[] chars;

    public String() {
        this.chars = "".toCharArray();
    }
    public String(char[] chars) {
        this.chars = chars.clone();
    }

    public int length() {
        return chars.length;
    }

    char[] toCharArray() {
        return chars;
    }

    public native String intern();
}