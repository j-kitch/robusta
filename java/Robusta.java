public class Robusta {

    private Robusta() {

    }

    public static native void println(String string);

    public static native boolean parseBoolean(String string);

    public static native byte parseByte(String string);

    public static native short parseShort(String string);

    public static native int parseInt(String string);

    public static native long parseLong(String string);

    public static native char parseChar(String string);

    public static native float parseFloat(String string);

    public static native double parseDouble(String string);
}
