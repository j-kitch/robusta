package java.lang;

import java.util.Arrays;

public class StringBuilder {

    private char[] chars;

    public StringBuilder() {
        chars = new char[0];
    }

    public StringBuilder append(String string) {
        if (string == null)
            return appendNull();
        int len = string.length();
        ensureCapacityInternal(chars.length + len);
        string.getChars(0, len, chars, chars.length);
        return this;
    }

    public StringBuilder appendNull() {
        return append("null");
    }

    public String toString() {
        return new String(chars);
    }

    private void ensureCapacityInternal(int length) {
        if (chars.length < length) {
            chars = Arrays.copyOf(chars, length);
        }
    }
}
