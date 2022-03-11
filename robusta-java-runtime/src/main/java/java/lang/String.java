package java.lang;

import java.util.Arrays;

public class String {

    private final char[] chars;

    public String() {
        chars = new char[0];
    }

    public String(char[] chars) {
        this.chars = Arrays.copyOf(chars, chars.length);
    }

    public void getChars(int srcBegin, int srcEnd, char[] dst, int dstBegin) {
        if (srcBegin < 0) {
            throw new StringIndexOutOfBoundsException(srcBegin);
        }
        if (srcEnd > chars.length) {
            throw new StringIndexOutOfBoundsException(srcEnd);
        }
        if (srcBegin > srcEnd) {
            throw new StringIndexOutOfBoundsException(srcEnd - srcBegin);
        }
        System.arraycopy(chars, srcBegin, dst, dstBegin, srcEnd - srcBegin);
    }

    public int length() {
        return chars.length;
    }
}
