package java.util;

public class Arrays {

    private Arrays() {

    }

    public static char[] copyOf(char[] original, int newLength) {
        char[] newArray = new char[newLength];
        System.arraycopy(original, 0, newArray, 0, original.length);
        return newArray;
    }
}
