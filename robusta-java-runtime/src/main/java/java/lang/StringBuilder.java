package java.lang;

import java.util.Arrays;

public class StringBuilder {

    private char[] chars;
    private int count;

    public StringBuilder() {
        chars = new char[0];
        count = 0;
    }

    public StringBuilder append(String string) {
        if (string == null)
            return appendNull();
        int len = string.length();
        ensureCapacityInternal(count + len);
        string.getChars(0, len, chars, count);
        count += len;
        return this;
    }

    public StringBuilder appendNull() {
        return append("null");
    }

    public String toString() {
        return new String(Arrays.copyOf(chars, count));
    }

    private void ensureCapacityInternal(int length) {
        if (chars.length < length) {
            chars = Arrays.copyOf(chars, length);
        }
    }
}
