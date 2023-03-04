public class Robusta {

    public static void internString(char[] chars) {
        String string = new String(chars);
        string.intern();
    }

    public static native void println(String string);

    public static native void println(int i);
}