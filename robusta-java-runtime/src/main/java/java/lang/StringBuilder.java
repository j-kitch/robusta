package java.lang;

import java.util.Arrays;

public class StringBuilder {

    private char[] value;
    private int count;

    public StringBuilder() {
        value = new char[0];
        count = 0;
    }

    public StringBuilder append(String string) {
        if (string == null)
            return appendNull();
        int len = string.length();
        ensureCapacityInternal(count + len);
        string.getChars(0, len, value, count);
        count += len;
        return this;
    }

    public StringBuilder append(boolean b) {
        return append(Boolean.toString(b));
    }

    public StringBuilder appendNull() {
        return append("null");
    }

    public String toString() {
        return new String(Arrays.copyOf(value, count));
    }

    private void ensureCapacityInternal(int length) {
        if (value.length < length) {
            value = Arrays.copyOf(value, length);
        }
    }
}
