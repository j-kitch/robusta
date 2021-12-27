package java.lang;

public class String {

    private final char[] chars;

    public String() {
        chars = new char[0];
    }

    private static native String fromConst(byte[] utf8);
}
