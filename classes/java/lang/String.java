package java.lang;

public class String {

    private char[] chars;

    public char[] getChars() {
        return chars;
    }

    private static native String fromUtf8(byte[] bytes);
}