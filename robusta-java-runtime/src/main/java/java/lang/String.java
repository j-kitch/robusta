package java.lang;

public class String {

    private final char[] chars;

    public String() {
        chars = new char[0];
    }

    public static String valueOf(Object obj) {
        return (obj == null) ? "null" : obj.toString();
    }
}
