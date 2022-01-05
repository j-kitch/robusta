public class ReturnValues {

    public static void main(String[] args) {
        System.out.println(getInt());
        System.out.println(getLong());
        System.out.println(getFloat());
        System.out.println(getDouble());
        System.out.println(getString());
    }

    private static int getInt() {
        return 10;
    }

    private static long getLong() {
        return 100;
    }

    private static float getFloat() {
        return 10.2f;
    }

    private static double getDouble() {
        return 123.456789;
    }

    private static String getString() {
        return "hello world";
    }
}
