package java.lang;

public class Integer implements Comparable<Integer> {

    private final int value;

    public Integer(int value) {
        this.value = value;
    }

    @Override
    public int compareTo(Integer o) {
        if (this.value < o.value) {
            return -1;
        } else if (this.value == o.value) {
            return 0;
        } else {
            return 1;
        }
    }

    public static native int parseInt(String s);

    public static Integer valueOf(int i) {
        return new Integer(i);
    }

    public static native String toString(int i);
}
