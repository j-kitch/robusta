package java.lang;

public class String {

    private char[] chars;

    public String() {
        this.chars = "".toCharArray();
    }
    public String(char[] chars) {
        this.chars = new char[chars.length];
        for (int i = 0; i < chars.length; i++) {
            this.chars[i] = chars[i];
        }
    }

    public int length() {
        return chars.length;
    }

    char[] toCharArray() {
        return chars;
    }

    public native String intern();
}